extern crate termcolor;
extern crate chrono;

mod conz;
mod parser;
mod data;
mod astr;

fn main() {
    let st = String::from("henlo ❤ world❤ test some more ❤ \t \n ai \t sir");
    println!("{}", st);
    let mut test = astr::from(st);
    println!("{}", astr::to_string(&test));
    let splitted = astr::split(&test, &astr::from(String::from(" \n\t")));
    for line in splitted{
        println!("{}", astr::to_string(&line));
    }
    println!("{}", astr::to_string(&test));
    astr::clear(&mut test);
    println!("{}", astr::to_string(&test));

    let printer = conz::Printer::new();
    let mut parser = parser::Parser::new(printer);
    parser.start_loop();
}
