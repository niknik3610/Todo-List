#[allow(dead_code)]     //TODO: remove
pub mod tui_handler {
    use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
    use crossterm::event as CEvent;
    use tui::{backend::CrosstermBackend, layout};
    use tui::{Terminal, widgets, style};
    use std::io::Stdout;
    use std::sync::{Arc, Mutex};
    use std::{
        sync::mpsc::channel,
        sync::mpsc::{Sender, Receiver},
        time::{Duration, Instant}, thread, io};
    use std::convert::From;


    const TICK_RATE: Duration = Duration::from_millis(200);
    const FRAME_RATE: u8 = 30;

    enum Event<T> {
        Input(T),
        Tick
    }

    enum MenuItem {
        Home
    }

    enum State {
        Viewing,
        DebugPrinting,
        Quitting,
    }

    enum UserAction<'a> {
        View,
        DebugMsg(&'a str),
        Quit,
    }

    impl From<MenuItem> for usize {
        fn from(input: MenuItem) -> usize {
            match input {
                MenuItem::Home => 0
            }
        }
    }

    pub fn run_tui() -> Result<(), String> {
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

        tui_handler(&rx, current_state).expect("TUI handler crashed");
        
        threads
            .into_iter()
            .for_each(|thread| thread.join().unwrap());
        return Ok(());
    }

    fn tui_handler(rx: &Receiver<Event<CEvent::KeyEvent>>, current_state: Arc<Mutex<State>>)-> Result <(), Box<dyn std::error::Error>> {
        let mut output_buffer: String = String::from("Buffer was never initilized");

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?; 
        let mut delta_time = Instant::now();


        loop {
            let input_result = match rx.recv().unwrap() {
                Event::Input(input) => handle_input(input),
                Event::Tick => continue
            };
            
            let mut current_state_data = current_state.lock().unwrap();
            match input_result {
                Some(result) => { 
                    match result {
                       UserAction::View => *current_state_data = State::Viewing,
                        UserAction::DebugMsg(msg) => {
                            *current_state_data = State::DebugPrinting;
                            output_buffer = msg.to_string();
                        }
                        UserAction::Quit => *current_state_data = State::Quitting
                    }
                }
                None => {}
            }
            
            if Instant::now().duration_since(delta_time) > Duration::from_millis(1000 / FRAME_RATE as u64) {
                delta_time = Instant::now();
                match *current_state_data {
                    State::Viewing => render_viewing(&mut terminal)?,
                    State::DebugPrinting => render_debugging(&mut terminal, &output_buffer)?,
                    State::Quitting => {
                        disable_raw_mode().unwrap();
                        return Ok(());
                   }
                }

                thread::sleep(Duration::from_millis(1000/(FRAME_RATE * 2) as u64))
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

    fn handle_input<'a>(input: CEvent::KeyEvent) -> Option<UserAction<'a>> { 
        use crossterm::event::KeyCode;
        let key = match input.code {
            KeyCode::Char(input) => input,  
            _ => return None
        };

        match key {
            'a' => Some(UserAction::DebugMsg("Hello World!")),
            'q' => Some(UserAction::Quit),
            _ => None 
        }
    }

    fn render_viewing(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|rec| {
            let size = rec.size();
            let chunks = layout::Layout::default()
                .direction(layout::Direction::Vertical)
                .margin(2)
                .constraints([
                             layout::Constraint::Length(3),  //Menu Bar
                             layout::Constraint::Min(2),     //Content
                             layout::Constraint::Length(3)   //Footer
                ]
                .as_ref()
                )
                .split(size);                   

            let msg = format!("Nik's Editor");
            let header =
                widgets::Paragraph::new(&msg[..])
                .style(style::Style::default().fg(style::Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .style(style::Style::default().fg(style::Color::White))
                    .border_type(widgets::BorderType::Plain)
                    );


            let content = 
                widgets::Paragraph::new("Todo:".to_owned()
                                        + "\nMake Minecraft      []"
                                        + "\nCelebrate Easter    []")
                .style(style::Style::default().fg(style::Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .style(style::Style::default().fg(style::Color::White))
                    .border_type(widgets::BorderType::Plain)
                    );

            let footer_copyright_temp = 
                widgets::Paragraph::new("Temp Copyright - Copyright Niklas Harnish")
                .style(style::Style::default().fg(style::Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .style(style::Style::default().fg(style::Color::White))
                    .title("Copyright")
                    .border_type(widgets::BorderType::Plain)
                    );

            rec.render_widget(header, chunks[0]);
            rec.render_widget(content, chunks[1]); 
            rec.render_widget(footer_copyright_temp, chunks[2]);
        }).expect("Drawing TUI");
        Ok(()) 
    }

    fn render_debugging(terminal: &mut Terminal<CrosstermBackend<Stdout>>, output_buffer: &String) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|rec| {
            let size = rec.size();
            let chunks = layout::Layout::default()
                .direction(layout::Direction::Vertical)
                .margin(2)
                .constraints([
                             layout::Constraint::Length(5),     //Debug
                             layout::Constraint::Length(3),     //Menu Bar
                             layout::Constraint::Min(2),        //Content
                             layout::Constraint::Length(3)      //Footer
                ]
                .as_ref()
                )
                .split(size);                   

            let debug =
                widgets::Paragraph::new(&output_buffer[..])
                .style(style::Style::default().fg(style::Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .style(style::Style::default().fg(style::Color::White))
                    .border_type(widgets::BorderType::Plain)
                    );


            let header =
                widgets::Paragraph::new("Nik's TODO List")
                .style(style::Style::default().fg(style::Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .style(style::Style::default().fg(style::Color::White))
                    .border_type(widgets::BorderType::Plain)
                    );


            let content = 
                widgets::Paragraph::new("Todo:".to_owned()
                                        + "\nMake Minecraft      []"
                                        + "\nCelebrate Easter    []")
                .style(style::Style::default().fg(style::Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .style(style::Style::default().fg(style::Color::White))
                    .border_type(widgets::BorderType::Plain)
                    );

            let footer_copyright_temp = 
                widgets::Paragraph::new("Temp Copyright - Copyright Niklas Harnish")
                .style(style::Style::default().fg(style::Color::LightCyan))
                .alignment(layout::Alignment::Center)
                .block(
                    widgets::Block::default()
                    .borders(widgets::Borders::ALL)
                    .style(style::Style::default().fg(style::Color::White))
                    .title("Copyright")
                    .border_type(widgets::BorderType::Plain)
                    );

            rec.render_widget(debug, chunks[0]);
            rec.render_widget(header, chunks[1]);
            rec.render_widget(content, chunks[2]); 
            rec.render_widget(footer_copyright_temp, chunks[3]);
        }).expect("Drawing TUI");
        Ok(()) 
    }
}
