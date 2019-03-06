extern crate termcolor;
extern crate chrono;

mod conz;
mod parser;
mod data;
mod astr;

fn main() {
    let st = String::from("henlo ❤ world❤");
    let mut test = astr::from(st);
    println!("{}", astr::to_string(test));
    
    let printer = conz::Printer::new();
    let mut parser = parser::Parser::new(printer);
    parser.start_loop();
}
