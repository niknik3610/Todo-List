#[allow(dead_code)]     //TODO: remove
mod todo_renderers;
pub mod tui_handler {
    use crate::todo_backend::todo::TodoList;
    use crate::tui_handler::todo_renderers::*;
    use crate::todo_backend::*;
    use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
    use crossterm::event as CEvent;
    use tui::backend::CrosstermBackend;
    use tui::Terminal;
    use std::sync::{Arc, Mutex};
    use std::{
        sync::mpsc::channel,
        sync::mpsc::{Sender, Receiver},
        time::{Duration, Instant}, thread, io
    };
    use std::convert::From;

    const TICK_RATE: Duration = Duration::from_millis(200);
    const FRAME_RATE: u8 = 30;

    enum Event<T> {
        Input(T),
        Tick,
    }

    enum State {
        Viewing,
        DebugPrinting,
        Quitting,
        AddingTodo,
    }

    enum UserAction<'a> {
        View,
        DebugMsg(&'a str),
        Quit,
        AddTodo,
        SubmitInput,
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
            let input_result = match rx.recv().unwrap() {
                Event::Input(input) => handle_input(input, &current_state),
                Event::Tick => continue,
            };
            { 
                let mut current_state_data = current_state.lock().unwrap();
                match input_result {
                    Some(result) => { 
                        match result {
                            UserAction::View => {
                                *current_state_data = State::Viewing;
                                output_buffer = String::from("");
                            },
                            UserAction::SubmitInput => {
                                *current_state_data = State::Viewing;
                                todo.add_item(&output_buffer)?;
                                todo_items = generate_todo(todo);
                                output_buffer = String::from("");
                            },
                            UserAction::DebugMsg(msg) => {
                                *current_state_data = State::DebugPrinting;
                                output_buffer = msg.to_string();
                            }
                            UserAction::Quit => *current_state_data = State::Quitting,
                            UserAction::AddTodo => *current_state_data = State::AddingTodo, 
                            UserAction::Input(input) => output_buffer.push(input),
                        }
                    }
                    None => {}
                } 
            }
            {
                let current_state_data = current_state.lock().unwrap();
                match *current_state_data {
                    State::Viewing => render_viewing(&mut terminal, &todo_items)?,
                    State::DebugPrinting => render_debugging(&mut terminal, &output_buffer, &todo_items)?,
                    State::AddingTodo => render_adding(&mut terminal, &output_buffer, &todo_items)?,
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

    fn handle_input<'a>(input: CEvent::KeyEvent, current_state: &Arc<Mutex<State>>) -> Option<UserAction<'a>> { 
        use crossterm::event::KeyCode;
        let current_state_data = current_state.lock().unwrap();

        if let State::Viewing = *current_state_data {
            let key = match input.code {    
                KeyCode::Char(input) => input,  
                _ => return None
            };

            return match key {
                'd' => Some(UserAction::DebugMsg("Hello World!")),
                'q' => Some(UserAction::Quit),
                'n' => Some(UserAction::AddTodo),
                _ => None 
            };    
        }

        match input.code {
            KeyCode::Char(input) => return Some(UserAction::Input(input)),
            KeyCode::Enter => return Some(UserAction::SubmitInput), 
            KeyCode::Esc => return Some(UserAction::View),
            _ => return None
        };  
    }

    fn generate_todo(todo: &TodoList) -> String {
        let mut todo_str = String::new();
        todo.items.iter().for_each(|(id, item)| {
            todo_str.push_str(&format!("{id} - {item_name} [{completed}]\n", item_name = item.title, completed = "")); 
        });
        
        return todo_str;
    }
}
