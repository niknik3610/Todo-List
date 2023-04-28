mod tui_buffer_handler;
mod tui_input_handler;
mod tui_rendering_handler;

pub mod tui_handler {
    use crate::todo_backend::todo::TodoList;
    use crate::tui_handler::{
        tui_buffer_handler as buffer, tui_input_handler as input, tui_rendering_handler as render,
    };
    use crossterm::event as CEvent;
    use crossterm::execute;
    use crossterm::terminal::{
        self as cTerm, disable_raw_mode, enable_raw_mode, LeaveAlternateScreen,
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
    const COMPLETED_ITEM: [char; 2] = [' ', '✓'];

    type ResultIo<T> = Result<T, io::Error>;

    pub enum Event<T> {
        Input(T),
        Tick,
    }

    pub enum State {
        Viewing,
        Quitting,
        AddingTodoDate(AddState),
        AddingTodo,
        CompletingTodo,
        UncompletingTodo,
        Error,
    }

    pub enum UserAction {
        Quit,
        AddTodoDate,
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
        EnteringDate(DateState),
    }

    #[derive(Copy, Clone)]
    pub enum DateState {
        Year,
        Month,
        Day,
        Time,
    }
    impl DateState {
        pub fn next(&self) -> Option<DateState> {
            match *self {
                DateState::Year => return Some(DateState::Month),
                DateState::Month => return Some(DateState::Day),
                DateState::Day => return Some(DateState::Time),
                DateState::Time => return None,
            }
        }
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

                    input::capture_input(&sx, &mut current_tick_time)
                        .expect("Input handler crashed");
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
        let mut todo_items = generate_todo(todo);

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Creating Terminal Failed");

        let mut user_input_buffer = String::new();
        let mut name_storage_buff = String::new();
        let mut date_storage_buff = String::new();

        render::render_main(&mut terminal, render::BufferType::None, &todo_items)?;
        loop {
            {
                let current_state = current_state.lock().unwrap();
                if let State::Quitting = *current_state {
                    return Ok(());
                }
            }

            //waits for user-input to render
            let input_result = match rx.recv()? {
                Event::Input(input) => input::handle_input(input, &current_state),
                Event::Tick => continue,
            };

            //semaphore for inputs
            let mut current_state = current_state.lock().unwrap();
            //would move into closure, but can't direct control flow from inside closure
            let input_result = match input_result {
                Ok(result) => result,
                Err(e) => {
                    handle_errors(e, &mut terminal, &todo_items)?;
                    continue;
                }
            };

            match input_result {
                //just change the state depending on user action
                UserAction::Quit => *current_state = State::Quitting,
                UserAction::AddTodo => *current_state = State::AddingTodo,
                UserAction::AddTodoDate => *current_state = State::AddingTodoDate(AddState::EnteringName),
                UserAction::CompeleteTodo => *current_state = State::CompletingTodo,
                UserAction::UncompleteTodo => *current_state = State::UncompletingTodo,
                UserAction::None => continue,
                UserAction::ManipulateBuffer(action) => {
                    let input_result = buffer::manipulate_buffer(
                        &mut *current_state,
                        action,
                        &mut user_input_buffer,
                        &mut name_storage_buff,
                        &mut date_storage_buff,
                        todo,
                        &mut todo_items,
                    );

                    match input_result {
                        Ok(()) => {}
                        Err(e) => {
                            handle_errors(e, &mut terminal, &todo_items)?;
                            *current_state = State::Viewing;
                            user_input_buffer = String::new();
                            date_storage_buff = String::new();
                            continue;
                        }
                    }
                }
            }
            //render the correct state
            render(
                &mut current_state,
                &user_input_buffer,
                &todo_items,
                &mut terminal,
                &name_storage_buff,
                &*date_storage_buff,
            )?;
        }
    }

    fn render(
        current_state: &mut std::sync::MutexGuard<State>,
        user_input_buffer: &String,
        todo_items: &String,
        mut terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        storage_buff: &String,
        date_storage_buff: &str,
    ) -> ResultIo<()> {
        match **current_state {
            State::Viewing => {
                render::render_main(&mut terminal, render::BufferType::None, &todo_items)?
            },
            State::AddingTodo => {
                render::render_adding(terminal, &user_input_buffer, todo_items)?
            },
            State::AddingTodoDate(state) => {
                use AddState::*;
                match state {
                    EnteringName => render::render_adding_date(
                        &mut terminal,
                        &*(user_input_buffer.to_owned() + "█"),
                        "",
                        "",
                        &todo_items,
                        &DateState::Year,
                    )?,
                    EnteringDate(state) => render::render_adding_date(
                        &mut terminal,
                        &storage_buff[..],
                        &*(user_input_buffer.to_owned() + "█"),
                        date_storage_buff,
                        &todo_items,
                        &state,
                    )?,
                }
            }
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
        return Ok(());
    }

    pub fn generate_todo(todo: &TodoList) -> String {
        let mut todo_str = String::from("   Todo:\n");
        let time_now = chrono::offset::Local::now();
        let mut timer: [i64; 3] = [0, 0, 0];

        todo.todo_items
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                todo_str.push_str(&format!(
                    "   {index} - {item_name} [{completed}]",
                    item_name = item.title,
                    completed = if !item.completed {
                        COMPLETED_ITEM[0]
                    } else {
                        COMPLETED_ITEM[1]
                    }
                ));

                if let Some(due) = item.due_date {
                    let due_duration = due
                        .signed_duration_since(time_now.naive_local())
                        .num_seconds();
                    timer = [
                        (due_duration / 60) % 60,      //mins
                        (due_duration / 60) / 60 % 24, //hrs
                        (due_duration / 60) / 60 / 24, //days
                    ];
                    todo_str.push_str(&format!(
                        " | Due: D:{d:0>2} H:{h:0>2} M:{m:0>2}",
                        d = timer[2],
                        h = timer[1],
                        m = timer[0],
                    ));

                    //todo_str.push_str(&format!(" | Due: {:?}", due_duration));
                }
                todo_str.push_str("\n");
            });

        todo_str.push_str("\n\n   Completed Todos:\n");
        todo.completed_items
            .iter()
            .enumerate()
            .for_each(|(index, item)| {
                todo_str.push_str(&format!(
                    "   {index} - {item_name} {completed}\n",
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

    pub fn handle_errors(
        e: io::Error,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        todo_items: &String,
    ) -> ResultIo<()> {
        use ErrorKind::*;
        match e.kind() {
            InvalidInput => {
                render::render_main(
                    terminal,
                    render::BufferType::Error("Invalid Input"),
                    todo_items,
                )?;
                return Ok(());
            }
            InvalidData => {
                render::render_main(
                    terminal,
                    render::BufferType::Error("Invalid or Empty Data"),
                    todo_items,
                )?;
                return Ok(());
            }
            Unsupported => {
                render::render_main(
                    terminal,
                    render::BufferType::Error("Invalid Date"),
                    todo_items,
                )?;
                return Ok(());
            }
            _ => return Err(e),
        }
    }
}
