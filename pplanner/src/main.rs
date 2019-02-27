extern crate termcolor;
extern crate chrono;

use termcolor::{ Color };

mod conz;
mod parser;

fn main() {
    let mut printer = conz::Printer::new();
    printer.set_color(Color::Green);
    let mut parser = parser::Parser::new(printer);
    parser.start_loop();
}
