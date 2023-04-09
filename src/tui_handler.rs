#[allow(dead_code)]     //TODO: remove
pub mod tui_handler {
    use crossterm::terminal::enable_raw_mode;
    use crossterm::event as CEvent;
    use tui::backend::CrosstermBackend;
    use tui::{Terminal, layout, layout::Direction, widgets, style};
    use std::{sync::mpsc::channel, sync::mpsc::Sender, time::{Duration, Instant}, thread, io};

    const TICK_RATE: Duration = Duration::from_millis(200);

    enum Event<T> {
        Input(T),
        Tick
    }

    enum MenuItem {
        Home
    }

    impl From<MenuItem> for usize {
        fn from(input: MenuItem) -> usize {
            match input {
                MenuItem::Home => 0
            }
        }
    }

    pub fn run_tui() -> Result<(), String> {
        enable_raw_mode().expect("Raw Mode");
        let (sx, _rx) = channel();
         
        thread::spawn(move || {
            input_handler(&sx).expect("Input handler crashed");
        });
        tui_handler().expect("TUI handler crashed");
        
        return Err("Something went wrong".to_owned());
    }

    fn tui_handler()-> Result <(), Box<dyn std::error::Error>> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?; 

        loop {
            terminal.draw(|rec| {
                let size = rec.size();
                let chunks = layout::Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([
                                 layout::Constraint::Length(3),  //Menu Bar
                                 layout::Constraint::Min(2),     //Content
                                 layout::Constraint::Length(3)   //Footer
                    ]
                    .as_ref()
                    )
                    .split(size);
                
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
                
                rec.render_widget(header, chunks[0]);
                rec.render_widget(content, chunks[1]); 
                rec.render_widget(footer_copyright_temp, chunks[2]);
            }).expect("Drawing TUI");
        }
    }

    fn input_handler(sx: &Sender<Event<CEvent::KeyEvent>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_tick_time = Instant::now();
        loop {
            let event_timer = TICK_RATE
                .checked_sub(current_tick_time.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if CEvent::poll(event_timer).expect("Polling Works") {
                if let CEvent::Event::Key(key) = CEvent::read().expect("Reading Events") {
                    sx.send(Event::Input(key)).expect("Sending Events");
                }
            }

            if current_tick_time.elapsed() >= TICK_RATE {
                match sx.send(Event::Tick) {
                    Ok(_) => current_tick_time = Instant::now(),
                    Err(e) => eprintln!("{e}") 
                }
            }
        }
    }
}

