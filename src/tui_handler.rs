mod todo_renderers;
pub mod tui_handler {
    use crate::todo_backend::todo::TodoList;
    use crate::tui_handler::todo_renderers::*;
    use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
    use crossterm::event as CEvent;
    use tui::backend::CrosstermBackend;
    use tui::Terminal;
    use std::error::Error;
    use std::io::ErrorKind;
    use std::sync::{Arc, Mutex};
    use std::{
        sync::mpsc::channel,
        sync::mpsc::{Sender, Receiver},
        time::{Duration, Instant}, 
        thread, 
        io
    };
    use std::convert::From;
    use std::io::Stdout;

    const TICK_RATE: Duration = Duration::from_millis(200);
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
        let current_state = Arc::new(Mutex::new(State::Viewing));
        let current_state_input = current_state.clone();

        enable_raw_mode().expect("Raw Mode");
        let (sx, rx) = channel();
        let mut threads = Vec::new(); 

        threads.push(thread::spawn(move|| {
            let mut current_tick_time = Instant::now();
            loop {
                {
                    //if panic I don't know how to recover
                    let current_state_data = current_state_input.lock().unwrap();
                    if let State::Quitting = *current_state_data  {
                        break;
                    }
                }
                capture_input(&sx, &mut current_tick_time).expect("Input handler crashed");     
            }
        }));

        tui_handler(&rx, current_state, todo_list).expect("TUI handler crashed");
        
        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap());
        return Ok(());
    }

    fn tui_handler(
        rx: &Receiver<Event<CEvent::KeyEvent>>, 
        current_state: Arc<Mutex<State>>,
        todo: &mut TodoList
        )-> Result <(), Box<dyn std::error::Error>> {

        let mut output_buffer = String::from("");
        let mut todo_items = generate_todo(todo);

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Creating Terminal Failed"); 

        render_viewing(&mut terminal, &todo_items).unwrap();
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
                            output_buffer = String::from("");
                        },
                        UserAction::Input(input) => output_buffer.push(input),
                        //handles user buffer input
                        UserAction::SubmitBuffer => {
                            submit_buffer(&current_state_data, &output_buffer[..], todo)?;
                            
                            *current_state_data = State::Viewing;
                            todo_items = generate_todo(todo);
                            output_buffer = String::from("");
                        },                         
                        UserAction::Backspace => {output_buffer.pop();},
                        //just change the state depending on user action
                        UserAction::Quit => *current_state_data = State::Quitting,
                        UserAction::AddTodo => *current_state_data = State::AddingTodo, 
                        UserAction::CompeleteTodo => *current_state_data = State::CompletingTodo,
                        UserAction::UncompleteTodo => *current_state_data = State::UncompletingTodo,
                        UserAction::None => continue
                    }
            }

            //render the correct state
            {
                let current_state_data = current_state.lock().unwrap();
                match *current_state_data {
                    State::Viewing => 
                        render_viewing(&mut terminal, &todo_items)?,
                    State::AddingTodo => 
                        render_with_buffer(&mut terminal, &output_buffer, &todo_items, "Adding: ")?,
                    State::CompletingTodo => 
                        render_with_buffer(&mut terminal, &output_buffer, &todo_items, "Completing: ")?,
                    State::UncompletingTodo =>
                        render_with_buffer(&mut terminal, &output_buffer, &todo_items, "Unclompleting: ")?, 
                    State::Quitting => {
                        disable_raw_mode().unwrap();
                        return Ok(());
                    }
                }
            }
        }
        
    }

    fn capture_input(
        sx: &Sender<Event<CEvent::KeyEvent>>, 
        current_tick_time: &mut Instant
        ) -> ResultIo<()> {
       
        let event_timer = TICK_RATE
            .checked_sub(current_tick_time.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if CEvent::poll(event_timer).expect("Polling Doesn't Work") {
            if let CEvent::Event::Key(key) = CEvent::read().expect("Reading Events Doesn't Work") {
                sx.send(Event::Input(key)).expect("Sending Events");
            }
        }

        if current_tick_time.elapsed() >= TICK_RATE {
            match sx.send(Event::Tick) {
                Ok(_) => *current_tick_time = Instant::now(),
                Err(e) => eprintln!("{e}") 
            }
        }

        Ok(()) 
    }

    fn handle_input(
        input: CEvent::KeyEvent,
        current_state: &Arc<Mutex<State>>) -> ResultIo<UserAction> { 
        use crossterm::event::KeyCode;
        let current_state_data = current_state.lock().unwrap();
        
        //handles user actions in normal mode
        if let State::Viewing = *current_state_data {
            let key = match input.code {    
                KeyCode::Char(input) => input,  
                _ => return Ok(UserAction::None)
            };

            return match key {
                'q' => Ok(UserAction::Quit),
                'n' => Ok(UserAction::AddTodo),
                'c' => Ok(UserAction::CompeleteTodo),
                'u' => Ok(UserAction::UncompleteTodo),
                _ => Ok(UserAction::None) 
            };    
        }
        
        //handles user actions when in buffer mode
        match input.code {
            KeyCode::Char(input) => return Ok(UserAction::Input(input)),
            KeyCode::Backspace => return Ok(UserAction::Backspace),
            KeyCode::Enter => return Ok(UserAction::SubmitBuffer), 
            KeyCode::Esc => return Ok(UserAction::View),
            _ => return Ok(UserAction::None)
        };  
    }

    fn generate_todo(todo: &TodoList) -> String {
        let mut todo_str = String::from("Todo:\n");
        todo.todo_items
            .iter()
            .enumerate()
            .for_each(|(index, item)| { 
                todo_str.push_str(&format!(
                        "{index} - {item_name} [{completed}]\n", 
                        item_name = item.title, 
                        completed = if !item.completed {COMPLETED_ITEM[0]} else {COMPLETED_ITEM[1]}
                        )); 
        });
        
        todo_str.push_str("\n\nCompleted Todos:\n");
        todo.completed_items
            .iter()
            .enumerate()
            .for_each(|(index, item)| { 
                todo_str.push_str(&format!(
                        "{index} - {item_name} [{completed}]\n", 
                        item_name = item.title, 
                        completed = if !item.completed {COMPLETED_ITEM[0]} else {COMPLETED_ITEM[1]}
                        )); 
        });
            
        
        return todo_str;
    }

    fn submit_buffer(
        current_state_data: &State, 
        output_buffer: &str,
        todo: &mut TodoList) -> ResultIo<()> {
            match *current_state_data {
            State::AddingTodo =>  {
                todo.add_item(&output_buffer)?;
            },
            State::CompletingTodo => {
                todo.complete_item(
                    output_buffer
                    .parse::<usize>()
                    .unwrap()).unwrap();
            }
            State::UncompletingTodo => {
                todo.uncomplete_item(
                    match output_buffer.parse::<usize>() {
                        Ok(r) => r,
                        Err(_) => return Err(ErrorKind::InvalidInput.into()) 
                    })?;
            }
            _ => {}
        }
        return Ok(())
    }

    fn handle_errors(e: io::Error, terminal: &mut Terminal<CrosstermBackend<Stdout>>, todo: &String) 
        -> ResultIo<()>{ 
            use ErrorKind::*;
            match e.kind() {
                InvalidInput => {
                    render_error(terminal, "Error: Invalid Input", &todo).unwrap();
                    return Ok(());
                }
                _ => return Err(ErrorKind::Other.into())
            }
    }
}
