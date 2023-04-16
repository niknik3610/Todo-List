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
There is currently only one mode: Command mode. By pressing the appropriate kindbind (view below), your keystrokes will be entered into a buffer in the command bar at the bottom of the screen. Press escape to delete the current buffer, or enter to complete your entry. These are the current keybinds (plan is for them to be remapable in the future):

- q: Quits the app and returns your terminal to the normal mode.
- n: Insert a new entry into your todo-list, by entering it's name into the buffer
- c: Completes a Todo item, enter the id of the item you want to complete into the buffer 
- u: Uncompletes a Todo item, enter the id of the item you want to uncomplete into the buffer 

## Dependencies
Tui-rs: https://crates.io/crates/serde \
Crossterm: https://crates.io/crates/crossterm \
Serde: https://crates.io/crates/serde

## References
https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
