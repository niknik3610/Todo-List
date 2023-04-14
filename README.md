Todo list written in Rust

# Building
Use Rust's cargo compiler to build the program. Further information can be found here: https://doc.rust-lang.org/cargo/getting-started/installation.html
Then run the command: 
```
cargo build --release
```

# Running
You can find the built app in the target/release directory within your project folder. Just open it through your terminal.

# KeyBinds
There are two modes currently: Insert and Viewing mode. Within insert and completion mode your keystrokes will be entered into a buffer in the header bar, press escape to leave Insert mode, or enter to complete your entry. Viewing mode has two keybinds at the moment:

- q: Quits the app and returns your terminal to the normal mode
- n: Insert a new entry into your todo-list, by entering it's name into the buffer
- c: Enter the id into the buffer, of the todo-item to complete the item

# Dependencies
Tui-rs: https://crates.io/crates/serde \
Crossterm: https://crates.io/crates/crossterm \
Serde: https://crates.io/crates/serde

# References
https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
