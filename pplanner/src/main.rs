extern crate termcolor;
extern crate chrono;

use termcolor::{ Color };

mod conz;
mod parser;

fn main() {
    let printer = conz::Printer::new();
    let mut parser = parser::Parser::new(printer);
    parser.start_loop();
}
