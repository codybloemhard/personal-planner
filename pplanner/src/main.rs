extern crate termcolor;
extern crate chrono;
extern crate dirs;
#[macro_use]
extern crate lazy_static;

mod conz;
mod parser;
mod data;
mod astr;
mod save;
mod wizard;
mod state;

fn main() {
    let mut printer = conz::Printer::new();
    let ok = save::setup_config_dir(&mut printer);
    if !ok {return;}
    let state = state::State::new();
    if state.is_none() {
        printer.println_type("Error: Could not create state.", conz::MsgType::Error);
        return;
    }
    let mut parser = parser::Parser::new(printer, state.unwrap());
    parser.start_loop();
}
