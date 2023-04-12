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
There are two modes currently: Insert and Viewing mode. Within insert mode your keystrokes will be registered in the header bar, press escape to leave Insert mode, or enter to complete your entry. Viewing mode has two keybinds at the moment:

#### 'q': quits the app and returns your terminal to the normal mode
#### 'n': insert a new entry into your todo-list
