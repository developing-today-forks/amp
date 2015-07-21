use application::Application;
use input::commands::{Command, application, workspace, cursor, buffer};
use rustbox::keyboard::Key;

pub fn handle(input: Key) -> Option<Command> {
    match input {
        Key::Char('q') => Some(buffer::close),
        Key::Char('Q') => Some(application::exit),
        Key::Char('j') => Some(cursor::move_down),
        Key::Char('k') => Some(cursor::move_up),
        Key::Char('h') => Some(cursor::move_left),
        Key::Char('l') => Some(cursor::move_right),
        Key::Char('x') => Some(buffer::delete),
        Key::Char('i') => Some(application::switch_to_insert_mode),
        Key::Char('s') => Some(buffer::save),
        Key::Char('H') => Some(cursor::move_to_start_of_line),
        Key::Char('L') => Some(cursor::move_to_end_of_line),
        Key::Char('I') => Some(cursor::insert_at_first_word_of_line),
        Key::Char('A') => Some(cursor::insert_at_end_of_line),
        Key::Char('o') => Some(cursor::insert_with_newline),
        Key::Char('f') => Some(application::switch_to_jump_mode),
        Key::Char('0') => Some(application::switch_to_open_mode),
        Key::Char('u') => Some(buffer::undo),
        Key::Char('r') => Some(buffer::redo),
        Key::Tab       => Some(workspace::next_buffer),
        _              => None,
    }
}
