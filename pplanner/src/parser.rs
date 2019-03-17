use std::collections::HashMap;

use termcolor::{ Color };

use super::conz;
use super::astr;
use super::astr::AStr;

pub struct Parser {
    funcs: HashMap<astr::Astr, fn(&mut conz::Printer, astr::AstrVec)>,
    printer: conz::Printer,
}

impl Parser {
    pub fn new(printer: conz::Printer) -> Parser {
        let mut funcs: HashMap<astr::Astr, fn(&mut conz::Printer, astr::AstrVec)> = HashMap::new();
        funcs.insert(astr::from_str("now"), commands::now);
        funcs.insert(astr::from_str("add"), commands::add_deadline);
        return Parser {
            funcs,
            printer,
        }
    }

    pub fn start_loop(&mut self) {
        self.printer.println_type("Henlo Fren!", conz::MsgType::Prompt);
        self.printer.println_type("pplanner: a ascii cli time management tool.", conz::MsgType::Prompt);
        self.printer.println_type("Made by Cody Bloemhard.", conz::MsgType::Prompt);
        loop{
            let x = conz::prompt(&mut self.printer, "cmd > ");
            let y = x.as_ref();
            match y {
                "q" => break,
                "quit" => break,
                _ => {
                    let found_cmd = self.parse_and_run(y);
                    if found_cmd { continue; }
                    self.printer.println_error("Error: Command not found: \"", y, "\"!");
                }
            }
        }
        self.printer.println_color("Bye!", Color::Cyan);
    }

    fn parse_and_run(&mut self, line: &str) -> bool{
        let command = astr::from_str(line).split_str(&astr::astr_whitespace());
        let search_res = self.funcs.get(&command[0]);
        match search_res {
            None => return false,
            Some(x) => x(&mut self.printer, command),
        }
        return true;
    }
}

mod commands {
    use super::super::conz;
    use super::super::data;
    use super::super::astr;
    use super::super::wizard;

    pub fn now(printer: &mut conz::Printer, _command: astr::AstrVec){
        let dt = data::DT::new();
        printer.println_type(dt.str_datetime().as_ref(), conz::MsgType::Value);
    }

    pub fn add_deadline(printer: &mut conz::Printer, _command: astr::AstrVec){
        let mut fields: Vec<wizard::Field> = Vec::new();
        fields.push(wizard::make_field(wizard::InputType::Text, astr::from_str("title: "), true));
        fields.push(wizard::make_field(wizard::InputType::DateTime, astr::from_str("deadline: "), true));
        let res = wizard::execute(&fields, printer);
        if res.is_err() { return; }
        let res = res.unwrap();
    }
}
