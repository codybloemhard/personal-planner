use std::collections::HashMap;
use termcolor::{ Color };

use super::conz;

pub struct Parser {
    funcs: HashMap<&'static str, fn(&mut conz::Printer)>,
    printer: conz::Printer,
}

impl Parser {
    pub fn new(printer: conz::Printer) -> Parser {
        let mut funcs: HashMap<&str, fn(&mut conz::Printer)> = HashMap::new();
        funcs.insert("now", commands::now);
        return Parser {
            funcs,
            printer,
        }
    }

    pub fn start_loop(&mut self) {
        loop{
            let x = conz::prompt(&mut self.printer, "cmd > ");
            let y = x.as_ref();
            match y {
                "q" => break,
                "quit" => break,
                _ => {
                    let found_cmd = self.parse_and_run(y);
                    if found_cmd { continue; }
                    self.printer.print_color("Error: Command not found: \"", Color::Red);
                    self.printer.print_color(y, Color::Green);
                    self.printer.println_color("\"!", Color::Red);
                }
            }
        }
        self.printer.println_color("Bye!", Color::Cyan);
    }

    fn parse_and_run(&mut self, line: &str) -> bool{
        let search_res = self.funcs.get(line);
        match search_res {
            None => return false,
            Some(x) => x(& mut self.printer),
        }
        return true;
    }
}

mod commands {
    use chrono::prelude::*;
    use termcolor::{ Color };
    use super::conz;

    pub fn now(printer : &mut conz::Printer) {
        printer.set_color(Color::Magenta);
        let dt = Local::now();
        println!("{}", dt);
    }
}