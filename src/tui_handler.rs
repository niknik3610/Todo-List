mod tui_rendering_handler;
mod tui_input_handler;
mod tui_buffer_handler;

pub mod tui_handler {
    use crate::todo_backend::todo::TodoList; 
    use crate::tui_handler::{
        tui_input_handler as input,
        tui_rendering_handler as render,
        tui_buffer_handler as buffer,
    };
    use crossterm::event as CEvent;
    use crossterm::execute;
    use crossterm::terminal::{
        self as cTerm, 
        disable_raw_mode, 
        enable_raw_mode,
        LeaveAlternateScreen
    };
    use std::convert::From;
    use std::io::stdout;
    use std::io::ErrorKind;
    use std::io::Stdout;
    use std::sync::{Arc, Mutex};
    use std::{
        io,
        sync::mpsc::channel,
        sync::mpsc::Receiver,
        thread,
        time::{Duration, Instant},
    };
    use tui::backend::CrosstermBackend;
    use tui::Terminal;

    pub const MAX_TICK_TIME: Duration = Duration::from_millis(200);
    const COMPLETED_ITEM: [char; 2] = [' ', 'X'];

    type ResultIo<T> = Result<T, io::Error>;

    pub enum Event<T> {
        Input(T),
        Tick,
    }

    pub enum State {
        Viewing,
        Quitting,
        AddingTodo(AddState),
        CompletingTodo,
        UncompletingTodo,
        Error,
    }

    pub enum UserAction {
        Quit,
        AddTodo,
        CompeleteTodo,
        UncompleteTodo,
        ManipulateBuffer(BufferAction),
        None,
    }

    pub enum BufferAction {
        Input(char),
        SubmitBuffer,
        Backspace,
        ExitBuffer,
    }
    
    #[derive(Copy, Clone)] 
    pub enum AddState {
        EnteringName,
        EnteringDate,
    }

    pub fn run_tui(todo_list: &mut TodoList) -> ResultIo<()> {
        enable_raw_mode().expect("Raw Mode");
        execute!(stdout(), cTerm::EnterAlternateScreen).unwrap();

        let current_state = Arc::new(Mutex::new(State::Viewing));

        let (sx, rx) = channel();
        let mut threads = Vec::new();

        //input thread and loop 
        {
            let current_state = current_state.clone();

            threads.push(thread::spawn(move || {
                let mut current_tick_time = Instant::now();
                loop {
                    let current_state = current_state.lock().unwrap();
                    
                    if let State::Quitting = *current_state {
                        break;
                    }
                    drop(current_state);

                    input::capture_input
                        (&sx, &mut current_tick_time).expect("Input handler crashed");
                }
            }));
        }

        let tui_result = tui_loop(&rx, &current_state, todo_list);
        
        {
            let mut current_state = current_state.lock().unwrap();
            *current_state = State::Quitting;
        }

        //exits gracefully on error
        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap());

        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();

        if let Err(e) = tui_result { 
            eprintln!("{:?}", e);
        }

        return Ok(());
    }

    fn tui_loop(
        rx: &Receiver<Event<CEvent::KeyEvent>>,
        current_state: &Arc<Mutex<State>>,
        todo: &mut TodoList,
        ) -> Result<(), Box<dyn std::error::Error>> { 
        let mut user_input_buffer = String::from("");
        let mut storage_buff = "".to_string();  
        let mut todo_items = generate_todo(todo);

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Creating Terminal Failed");

        render::render_main(&mut terminal, render::BufferType::None, &todo_items)?;
        loop {
            {
                let current_state = current_state.lock().unwrap();
                if let State::Quitting = *current_state {
                    return Ok(());
                }
            }

            //waits for user-input to render
            let input_result = match rx.recv()?{
                Event::Input(input) => input::handle_input(input, &current_state),
                Event::Tick => continue,
            };

            //deals with key events
            let mut current_state = current_state.lock().unwrap();

            let result = match input_result {
                Ok(result) => result,
                Err(e) => {
                    handle_errors(e, &mut terminal, &todo_items)?;
                    continue;
                }
            };

            match result { 
                //just change the state depending on user action
                UserAction::Quit => *current_state = State::Quitting,
                UserAction::AddTodo => *current_state = State::AddingTodo(AddState::EnteringName),
                UserAction::CompeleteTodo => *current_state = State::CompletingTodo,
                UserAction::UncompleteTodo => *current_state = State::UncompletingTodo,
                UserAction::None => continue,
                UserAction::ManipulateBuffer(action) => {
                    match action {
                        BufferAction::Input(input) => user_input_buffer.push(input),
                        BufferAction::Backspace => {user_input_buffer.pop();} 
                        BufferAction::ExitBuffer => {
                            *current_state = State::Viewing;
                            user_input_buffer = String::new();
                        }
                        BufferAction::SubmitBuffer => {
                            match *current_state {
                                State::AddingTodo(AddState::EnteringName) => {
                                    buffer::swap_buffers(&user_input_buffer, &mut storage_buff)?;
                                    user_input_buffer = String::new(); 
                                    *current_state = State::AddingTodo(AddState::EnteringDate);
                                }, 
                                _ => { 
                                    let submit_result = 
                                        buffer::submit_buffer
                                        (&current_state, &*storage_buff, &user_input_buffer, todo); 

                                    *current_state = State::Viewing;
                                    todo_items = generate_todo(todo);
                                    user_input_buffer = String::from("");

                                    if let Err(e) = submit_result {
                                        handle_errors(e, &mut terminal, &todo_items)?;
                                        *current_state = State::Viewing;
                                        user_input_buffer = String::new();
                                        continue;
                                    }
                                }
                            };
                        }
                    }
                }, 
            }
            //render the correct state
            render(&mut current_state, &user_input_buffer, &todo_items, &mut terminal, &storage_buff)?;
        }
    }

    fn render(
            current_state: &mut std::sync::MutexGuard<State>,
            user_input_buffer: &String,
            todo_items: &String,
            mut terminal: &mut Terminal<CrosstermBackend<Stdout>>,
            storage_buff: &String
        ) -> ResultIo<()> {
        match **current_state {
            State::Viewing => 
                render::render_main(&mut terminal, render::BufferType::None, &todo_items)?,
            State::AddingTodo(state)  => {
                use AddState::*;
                match state {
                    EnteringName => render::render_adding(
                        &mut terminal,
                        &*(user_input_buffer.to_owned() + "█"),
                        "",
                        &todo_items,
                        )?,
                    EnteringDate => render::render_adding(
                        &mut terminal,
                        &storage_buff[..],
                        &*(user_input_buffer.to_owned() + "█"),
                        &todo_items,
                        )?,
                }
            },
            State::CompletingTodo => render::render_main(
                &mut terminal,
                render::BufferType::CompletingTask(&user_input_buffer),
                &todo_items,
                )?,
            State::UncompletingTodo => render::render_main(
                &mut terminal,
                render::BufferType::UncompletingTask(&user_input_buffer),
                &todo_items,
                )?,
            State::Error => **current_state = State::Viewing,
            State::Quitting => {
                return Ok(());
            }
        }
        return Ok(())
    }

    fn generate_todo(todo: &TodoList) -> String {
        let mut todo_str = String::from("Todo:\n");
        let time_now = chrono::offset::Local::now();
        let mut timer: [i64; 3] = [0, 0, 0];

        todo.todo_items
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                todo_str.push_str(&format!(
                    "{index} - {item_name} [{completed}]",
                    item_name = item.title,
                    completed = if !item.completed {
                        COMPLETED_ITEM[0]
                    } else {
                        COMPLETED_ITEM[1]
                    } 
                ));

                if let Some(due) = item.due_date {
                    let due_duration = due.signed_duration_since(time_now.naive_local()).num_seconds();
                    timer = [
                        (due_duration / 60) % 60,       //mins
                        (due_duration / 60) / 60 % 24,  //hrs
                        (due_duration / 60) / 60 / 24   //days
                            ];
                    todo_str.push_str(&format!(" | Due: D:{d:0>2} H:{h:0>2} M:{m:0>2}",
                                               d = timer[2],
                                               h = timer[1],
                                               m = timer[0],
                                               ));
                                               
                    //todo_str.push_str(&format!(" | Due: {:?}", due_duration));
                }
                todo_str.push_str("\n");
            });

        todo_str.push_str("\n\nCompleted Todos:\n");
        todo.completed_items
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                todo_str.push_str(&format!(
                    "{index} - {item_name} [{completed}]\n",
                    item_name = item.title,
                    completed = if !item.completed {
                        COMPLETED_ITEM[0]
                    } else {
                        COMPLETED_ITEM[1]
                    }
                ));
            });

        return todo_str;
    } 

    fn handle_errors(
        e: io::Error,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        todo_items: &String,
    ) -> ResultIo<()> {
        use ErrorKind::*;
        match e.kind() {
            InvalidInput => {
                render::render_main(terminal, render::BufferType::Error("Invalid Input"), todo_items)?;
                return Ok(());
            }
            InvalidData => { 
                render::render_main(terminal, render::BufferType::Error("Invalid or Empty Data"), todo_items)?;
                return Ok(());
            }
            Unsupported => { 
                render::render_main(terminal, render::BufferType::Error("Invalid Date"), todo_items)?;
                return Ok(()); 
            }
            _ => return Err(e),
        }
    }
}
