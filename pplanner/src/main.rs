extern crate termcolor;
extern crate chrono;
extern crate dirs;
#[macro_use]
extern crate lazy_static;
extern crate num_derive;

mod conz;
mod parser;
mod data;
mod astr;
mod save;
mod wizard;
mod state;

use conz::PrinterFunctions;

fn main() {
    let ok = save::setup_config_dir();
    if !ok {return;}
    let state = state::State::new();
    if state.is_none() {
        conz::printer().println_type(&"Error: Could not create state.", conz::MsgType::Error);
        return;
    }
    let mut parser = parser::Parser::new(state.unwrap());
    parser.start_loop();
}
