
pub mod tui_handler {
    use crossterm::terminal::enable_raw_mode;
    use crossterm::event::Event as CEvent;
    use crossterm::event;
    use tui::backend::CrosstermBackend;
    use tui::{Terminal, layout::Layout, layout::Direction};
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

    fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode().expect("Raw Mode");
        let (sx, rx) = channel();
        
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?; 

        thread::spawn(move || {
            input_handler(sx);
        });
        
        return Err(Error);
    }

    fn tui_renderer() {
        loop {
            terminal.draw(|rec| {
                let size = rec.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical);
            })
        }
    }

    fn input_handler(sx: &Sender<Event>) -> Result<(), Box<dyn std::error::Error>> {
        let mut current_tick_time = Instant::now();
        loop {
            let event_timer = TICK_RATE
                .checked_sub(current_tick_time.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(event_timer).expect("Polling Works") {
                if let CEvent::Key(key) = event::read().expect("Reading Events") {
                    sx.send(Event::Input(key)).expect("Sending Events");
                }
            }

            if current_tick_time.elapsed() >= TICK_RATE {
                sx.send(Event::Tick).expect("Sending Events");
                Ok(());
            }
        }
    }
}

