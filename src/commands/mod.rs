use crate::errors;
use crate::models::application::Application;
use crate::view::Terminal;
use std::collections::HashMap;

pub mod application;
pub mod buffer;
pub mod confirm;
pub mod cursor;
pub mod git;
pub mod jump;
pub mod line_jump;
pub mod path;
pub mod preferences;
pub mod search;
pub mod selection;
pub mod search_select;
pub mod view;
pub mod workspace;

pub type Command = fn(&mut Application) -> Result;
pub type Result = errors::Result<()>;

pub fn hash_map() -> HashMap<&'static str, Command> {
    include!(concat!(env!("OUT_DIR"), "/hash_map"))
}

