mod todo_renderers;
pub mod tui_handler {
    use crate::todo_backend::todo::TodoList;
    use crate::tui_handler::todo_renderers::*;

    use chrono::TimeZone;
    use chrono::Utc;
    use chrono::format::format;
    use crossterm::event as CEvent;
    use crossterm::execute;
    use crossterm::terminal::LeaveAlternateScreen;
    use crossterm::terminal::{self as cTerm, disable_raw_mode, enable_raw_mode};
    use std::convert::From;
    use std::io::stdout;
    use std::io::ErrorKind;
    use std::io::Stdout;
    use std::sync::{Arc, Mutex};
    use std::{
        io,
        sync::mpsc::channel,
        sync::mpsc::{Receiver, Sender},
        thread,
        time::{Duration, Instant},
    };
    use tui::backend::CrosstermBackend;
    use tui::Terminal;

    const MAX_TICK_TIME: Duration = Duration::from_millis(200);
    const COMPLETED_ITEM: [char; 2] = [' ', 'X'];

    type ResultIo<T> = Result<T, io::Error>;

    enum Event<T> {
        Input(T),
        Tick,
    }

    enum State {
        Viewing,
        Quitting,
        AddingTodo,
        CompletingTodo,
        UncompletingTodo,
        Error,
    }

    enum UserAction {
        View,
        Quit,
        AddTodo,
        CompeleteTodo,
        UncompleteTodo,
        //input Actions
        Input(char),
        SubmitBuffer,
        Backspace,
        None,
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
                    capture_input(&sx, &mut current_tick_time).expect("Input handler crashed");
                }
            }));
        }
        
        let tui_result = tui_loop(&rx, &current_state, todo_list);

        //exits gracefully on error
        if let Err(e) = tui_result { 
            eprintln!("{:?}", e);

            let mut current_state = current_state.lock().unwrap();
            *current_state = State::Quitting;
        }

        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap());
        
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        return Ok(());
    }

    fn tui_loop(
        rx: &Receiver<Event<CEvent::KeyEvent>>,
        current_state: &Arc<Mutex<State>>,
        todo: &mut TodoList,
        ) -> Result<(), Box<dyn std::error::Error>> { 
        let mut user_input_buffer = String::from("");
        let mut todo_items = generate_todo(todo);

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Creating Terminal Failed");

        render_main(&mut terminal, BufferType::None, &todo_items).unwrap();
        loop {
            //waits for user-input to render
            let input_result = match rx.recv().unwrap() {
                Event::Input(input) => handle_input(input, &current_state),
                Event::Tick => continue,
            };

            //deals with key events
            {
                let mut current_state_data = current_state.lock().unwrap();

                let result = match input_result {
                    Ok(result) => result,
                    Err(e) => {
                        handle_errors(e, &mut terminal, &todo_items)?;
                        continue;
                    }
                };

                match result {
                    UserAction::View => {
                        *current_state_data = State::Viewing;
                        user_input_buffer = String::new();
                    }
                    UserAction::Input(input) => user_input_buffer.push(input),
                    //handles user buffer input
                    UserAction::SubmitBuffer => {
                        if let Err(e) = submit_buffer(&current_state_data, &user_input_buffer[..], todo) {
                            handle_errors(e, &mut terminal, &todo_items)?;
                            *current_state_data = State::Viewing;
                            user_input_buffer = String::new();
                            continue;
                        }

                        *current_state_data = State::Viewing;
                        todo_items = generate_todo(todo);
                        user_input_buffer = String::from("");
                    }
                    UserAction::Backspace => {
                        user_input_buffer.pop();
                    }
                    //just change the state depending on user action
                    UserAction::Quit => *current_state_data = State::Quitting,
                    UserAction::AddTodo => *current_state_data = State::AddingTodo,
                    UserAction::CompeleteTodo => *current_state_data = State::CompletingTodo,
                    UserAction::UncompleteTodo => *current_state_data = State::UncompletingTodo,
                    UserAction::None => continue,
                }
            }

            //render the correct state
            {
                let mut current_state_data = current_state.lock().unwrap();
                match *current_state_data {
                    State::Viewing => render_main(&mut terminal, BufferType::None, &todo_items)?,
                    State::AddingTodo => render_adding(
                        &mut terminal,
                        user_input_buffer.as_str(),
                        &todo_items,
                        )?,
                    State::CompletingTodo => render_main(
                        &mut terminal,
                        BufferType::CompletingTask(&user_input_buffer),
                        &todo_items,
                        )?,
                    State::UncompletingTodo => render_main(
                        &mut terminal,
                        BufferType::UncompletingTask(&user_input_buffer),
                        &todo_items,
                        )?,
                    State::Error => *current_state_data = State::Viewing,
                    State::Quitting => {
                        return Ok(());
                    }
                }
            }
        }
    }

    fn capture_input(
        sx: &Sender<Event<CEvent::KeyEvent>>,
        current_tick_time: &mut Instant,
    ) -> ResultIo<()> {
        let event_timer = MAX_TICK_TIME
            .checked_sub(current_tick_time.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if CEvent::poll(event_timer).expect("Polling Doesn't Work") {
            if let CEvent::Event::Key(key) = CEvent::read().expect("Reading Events Doesn't Work") {
                sx.send(Event::Input(key)).expect("Sending Events");
            }
        }

        if current_tick_time.elapsed() >= MAX_TICK_TIME {
            match sx.send(Event::Tick) {
                Ok(_) => *current_tick_time = Instant::now(),
                Err(e) => eprintln!("{e}"),
            }
        }

        Ok(())
    }

    fn handle_input(
        input: CEvent::KeyEvent,
        current_state: &Arc<Mutex<State>>,
    ) -> ResultIo<UserAction> {
        use crossterm::event::KeyCode;
        let current_state_data = current_state.lock().unwrap();

        //handles user actions in normal mode
        if let State::Viewing = *current_state_data {
            let key = match input.code {
                KeyCode::Char(input) => input,
                _ => return Ok(UserAction::None),
            };

            return match key {
                'q' => Ok(UserAction::Quit),
                'n' => Ok(UserAction::AddTodo),
                'c' => Ok(UserAction::CompeleteTodo),
                'u' => Ok(UserAction::UncompleteTodo),
                _ => Ok(UserAction::None),
            };
        }

        //handles user actions when in buffer mode
        match input.code {
            KeyCode::Char(input) => return Ok(UserAction::Input(input)),
            KeyCode::Backspace => return Ok(UserAction::Backspace),
            KeyCode::Enter => return Ok(UserAction::SubmitBuffer),
            KeyCode::Esc => return Ok(UserAction::View),
            _ => return Ok(UserAction::None),
        };
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
                    let due_duration = due.signed_duration_since(time_now).num_seconds();
                    timer = [
                        (due_duration / 60) % 60,       //mins
                        (due_duration / 60) / 60,       //hrs
                        (due_duration / 60) / 60 / 24   //days
                            ];
                    todo_str.push_str(&format!(" | Due: {d:0>2}:{h:0>2}:{m:0>2}",
                                               d = timer[2],
                                               h = timer[1],
                                               m = timer[0],
                                               ));
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

    fn submit_buffer(
        current_state_data: &State,
        output_buffer: &str,
        todo: &mut TodoList,
    ) -> ResultIo<()> {
        if let State::AddingTodo = *current_state_data { 
            todo.add_item(&output_buffer)?;
            return Ok(());
        }
        
        let output_buffer = match output_buffer.parse::<usize>() {
            Ok(r) => r,
            Err(_) => return Err(ErrorKind::InvalidInput.into())
        }; 

        match *current_state_data {
            State::CompletingTodo => {
                if output_buffer > todo.todo_len() - 1 {
                    return Err(ErrorKind::InvalidInput.into());
                }
                todo.complete_item(output_buffer)?;
            },
            State::UncompletingTodo => {
                if output_buffer > todo.completed_len() - 1 {
                    return Err(ErrorKind::InvalidInput.into());
                }
                todo.uncomplete_item(output_buffer)?;
            }
            _ => {}
        }

        return Ok(());
    }

    fn handle_errors(
        e: io::Error,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        todo_items: &String,
    ) -> ResultIo<()> {
        use ErrorKind::*;
        match e.kind() {
            InvalidInput => {
                render_main(terminal, BufferType::Error("Invalid Input"), todo_items).unwrap();
                return Ok(());
            }
            InvalidData => { 
                render_main(terminal, BufferType::Error("Invalid or Empty Data"), todo_items).unwrap();
                return Ok(());
            }
            _ => return Err(e),
        }
    }
}
