# password generator

A tui generating passwords of different types

- pin
- random (configurable to include numbers and symbols)

Users can copy the generated password to the clipboard

![a demonstration of the app. the user navigates to random page and toggles the number and symbols option then creates a pin](demo.gif)

## Usage

This tui uses `crossterm` as a backend, so it should support most terminals. Check [crossterm's tested terminals](https://github.com/crossterm-rs/crossterm?tab=readme-ov-file#tested-terminals) for compatibility.

Install Rust and run `cargo run` or build a binary with `cargo build --release` and run the binary in the `target/release` directory.

## Roadmap

- [x] pin password type
- [x] random password type
- [x] toggleable numbers and symbols for random password
- [x] vim keybindings
- [x] copy password to clipboard
- [ ] ctrl + c to exit
- [ ] password strength gauge
- [ ] different colors for symbols, numbers, and letters
- [ ] wrap commands list for smaller terminal windows

## What I learned

TUIs state management made me appreciate how much the browser gives us for free. I was not used to manually managing state like the current page or toggling options

I chose Ratatui for this project, which uses immediate–mode rendering, meaning the user needs to handle the event loop and the UI is redrawn on every event

Ratatui is definitely manageable for this password manager, but it's better suited for a flatter, less interactive applications like dashboards

Each submenu requires more nested state since state must be created outside of the event loop, which is much less ergonomic that most web frameworks, where events and state can easily be encapsulated in a component

I don't necessarily dislike ratatui, but I wonder what it would be like to write a TUI with state local to a widget

### Alternative crates for the future

I'd like to use a TUI framework with nicer state management in the future. It was a good experience to handle state more manually, but I'd like to focus less on state management and more on application logic

#### Stable

- [tuirealm](https://crates.io/crates/tuirealm) A react and elm based framework for ratatui. Nice features include lots of components, including checkbox groups and select!
- [r3bl_tui](https://crates.io/crates/r3bl_tui) A react inspired tui framework, but async and with utilities like redux!
- [cursive](https://github.com/gyscos/Cursive) a separate TUI framework that handles the event loop for you.
- [widgetui](https://crates.io/crates/tui-react) is a wrapper around ratatui to cut down on boilerplate. More of a companion than an alternative since it's compatible with ratatui!

#### experimental

- [tui-react](https://crates.io/crates/tui-react) is a library made by someone with the same issue I had! It allows for stateful components by allowing any struct to render to the terminal. It even has the `List` with generic items I wanted :)
- [dioxus-tui](https://crates.io/crates/dioxus-tui). Actually just use web semantics in the terminal
