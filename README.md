# A Todo list written in Rust
Written by Niklas Harnish

## Building
Use Rust's cargo compiler to build the program. Further information can be found here: https://doc.rust-lang.org/cargo/getting-started/installation.html
Then run the command: 
```
cargo build --release
```

## Running
You can find the built app in the target/release directory within your project folder. Just open it through your terminal.

## KeyBinds
There are currently only two modes: Command mode and new Task Mode. In command mode, by pressing the appropriate kindbind (view below), your keystrokes will be entered into a buffer in the command bar at the bottom of the screen. Press escape to delete the current buffer, or enter to complete your entry. New Task mode lets you create a new task in the window.

These are the current keybinds available in command mode (plan is for them to be remapable in the future):
- ":": Enter Command Mode (View Commands Section Below).
- q: Quits the app and returns your terminal to the normal mode.
- n: Insert a new entry into your todo-list, by entering it's name into the buffer.
- d: Insert a new entry, with a date, into your todo-list. View the date Section below.
- c: Completes a Todo item, enter the id of the item you want to complete into the buffer.
- u: Uncompletes a Todo item, enter the id of the item you want to uncomplete into the buffer. 

## Commands:
- 'AddTask'
- 'AddTaskDate'
- 'CompleteTask' 
- 'UncompleteTask'
- 'Quit'

## Dates
Currently there are some issues with Date formatting, requiring you to be very specific with your input for it to be correctly parsed. The format should be: 
Full-year (ex: 2023), Short-name for month (ex: Mar), Full-day (ex: 23), hours:minutes:seconds (12:00:00).\
Full input might look like: May 12 2023 12:00:00 \
I will attempt to make it more user friendly in the future. 

## Dependencies
Tui-rs: https://crates.io/crates/serde \
Crossterm: https://crates.io/crates/crossterm \
Serde: https://crates.io/crates/serde

## References
https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
