extern crate scribe;
extern crate rustbox;
extern crate pad;

use application::Mode;

mod application;
mod terminal;
mod view;
mod input;

fn main() {
    let mut application = application::new();
    let terminal = terminal::new();
    let mut view = view::new(&terminal);

    // Set the view's initial status line.
    match application.workspace.current_buffer().unwrap().file_name() {
        Some(file_name) => view.status_line = file_name,
        None => (),
    }

    loop {
        // Refresh the text and cursor on-screen.
        view.set_cursor(&terminal, &*application.workspace.current_buffer().unwrap().cursor);
        let tokens = match application.mode {
            Mode::Jump(ref mut jump_mode) => {
                let visible_lines = view.visible_lines();
                jump_mode.tokens(
                    &application.workspace.current_buffer().unwrap().tokens(),
                    Some(visible_lines)
                )
            },
            _ => {
                application.workspace.current_buffer().unwrap().tokens()
            },
        };

        match application.mode {
            Mode::Insert(_) => view.display(&terminal, &tokens, true),
            Mode::Open(ref open_mode) => view::open::display(&terminal, &tokens, open_mode),
            _ => view.display(&terminal, &tokens, false),
        };

        match terminal.get_input() {
            Some(key) => {
                // Pass the input to the current mode.
                let command = match application.mode {
                    Mode::Normal => input::modes::normal::handle(key),
                    Mode::Insert(ref mut i) => input::modes::insert::handle(i, key),
                    Mode::Jump(ref mut j) => input::modes::jump::handle(j, key),
                    Mode::Open(ref mut o) => input::modes::open::handle(o, key),
                    Mode::Exit => break,
                };

                // If the current mode returned a command, run it.
                match command {
                    Some(c) => c(&mut application),
                    None => (),
                }

                // Check if the command resulted in an exit, before
                // looping again and asking for input we won't use.
                match application.mode {
                    Mode::Exit => break,
                    _ => {}
                }
            },
            None => {},
        }
    }
}
