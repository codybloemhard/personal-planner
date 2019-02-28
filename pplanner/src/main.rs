extern crate termcolor;
extern crate chrono;

mod conz;
mod parser;
mod data;

fn main() {
    let printer = conz::Printer::new();
    let mut parser = parser::Parser::new(printer);
    parser.start_loop();
}
