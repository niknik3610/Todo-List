use std::sync::mpsc::Sender;

use super::tui_handler::MAX_TICK_TIME;
use super::tui_handler::*;
use crossterm::event as CEvent;
use std::sync::{Arc, Mutex};
use std::{
    io,
    time::{Duration, Instant},
};

pub fn capture_input(
    sx: &Sender<Event<CEvent::KeyEvent>>,
    current_tick_time: &mut Instant,
) -> io::Result<()> {
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

pub fn handle_input(
    input: CEvent::KeyEvent,
    current_state: &Arc<Mutex<State>>,
) -> io::Result<UserAction> {
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
    use BufferAction::*;
    match input.code {
        KeyCode::Char(input) => return Ok(UserAction::ManipulateBuffer(Input(input))),
        KeyCode::Backspace => return Ok(UserAction::ManipulateBuffer(Backspace)),
        KeyCode::Enter => return Ok(UserAction::ManipulateBuffer(SubmitBuffer)),
        KeyCode::Esc => return Ok(UserAction::ManipulateBuffer(ExitBuffer)),
        _ => return Ok(UserAction::None),
    };
}
