#[allow(dead_code)]     //TODO: remove
mod todo_renderers;
pub mod tui_handler {
    use crate::todo_backend::todo::TodoList;
    use crate::tui_handler::todo_renderers::*;
    use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
    use crossterm::event as CEvent;
    use tui::backend::CrosstermBackend;
    use tui::Terminal;
    use std::sync::{Arc, Mutex};
    use std::{
        sync::mpsc::channel,
        sync::mpsc::{Sender, Receiver},
        time::{Duration, Instant}, 
        thread, 
        io
    };
    use std::convert::From;

    const TICK_RATE: Duration = Duration::from_millis(200);
    const COMPLETED_ITEM: [char; 2] = [' ', 'X'];

    enum Event<T> {
        Input(T),
        Tick,
    }

    enum State {
        Viewing,
        Quitting,
        AddingTodo,
        CompletingTodo,
    }

    enum UserAction {
        View,
        Quit,
        AddTodo,
        CompeleteTodo,
        SubmitBuffer,
        Input(char),
    }

    pub fn run_tui(todo_list: &mut TodoList) -> Result<(), String> {
        let current_state = Arc::new(Mutex::new(State::Viewing));
        let current_state_input = current_state.clone();

        enable_raw_mode().expect("Raw Mode");
        let (sx, rx) = channel();
        let mut threads = Vec::new(); 

        threads.push(thread::spawn(move|| {
            let mut current_tick_time = Instant::now();
            loop {
                {
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

    fn tui_handler(rx: &Receiver<Event<CEvent::KeyEvent>>, current_state: Arc<Mutex<State>>, todo: &mut TodoList)-> Result <(), Box<dyn std::error::Error>> {
        let mut output_buffer = String::from("");
        let mut todo_items = generate_todo(todo);

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?; 

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

                if let Some(result) = input_result {
                    match result {
                        UserAction::View => {
                            *current_state_data = State::Viewing;
                            output_buffer = String::from("");
                        },
                        //handles user buffer input
                        UserAction::SubmitBuffer => {
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
                                _ => {}
                            }
                            *current_state_data = State::Viewing;
                            todo_items = generate_todo(todo);
                            output_buffer = String::from("");
                        }, 
                        UserAction::Quit => *current_state_data = State::Quitting,
                        UserAction::AddTodo => *current_state_data = State::AddingTodo, 
                        UserAction::CompeleteTodo => *current_state_data = State::CompletingTodo,
                        UserAction::Input(input) => output_buffer.push(input),
                    }
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
                    State::Quitting => {
                        disable_raw_mode().unwrap();
                        return Ok(());
                    }
                }
            }
        }
        
    }

    fn capture_input(sx: &Sender<Event<CEvent::KeyEvent>>, current_tick_time: &mut Instant) -> Result<(), Box<dyn std::error::Error>> {
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

    fn handle_input<'a>(input: CEvent::KeyEvent, current_state: &Arc<Mutex<State>>) -> Option<UserAction> { 
        use crossterm::event::KeyCode;
        let current_state_data = current_state.lock().unwrap();
        
        //handles user actions in normal mode
        if let State::Viewing = *current_state_data {
            let key = match input.code {    
                KeyCode::Char(input) => input,  
                _ => return None
            };

            return match key {
                'q' => Some(UserAction::Quit),
                'n' => Some(UserAction::AddTodo),
                'c' => Some(UserAction::CompeleteTodo),
                _ => None 
            };    
        }
        
        //handles user actions when in buffer mode
        match input.code {
            KeyCode::Char(input) => return Some(UserAction::Input(input)),
            KeyCode::Enter => return Some(UserAction::SubmitBuffer), 
            KeyCode::Esc => return Some(UserAction::View),
            _ => return None
        };  
    }

    fn generate_todo(todo: &TodoList) -> String {
        let mut todo_str = String::new();
        todo.items
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
}
