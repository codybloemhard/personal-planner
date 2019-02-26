use std::io;
use std::io::Write; //flush stdout
extern crate termcolor;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

struct Printer{
    stream: StandardStream,
    col: Color,
}

impl Printer {
    fn set_color(&mut self, color: Color){
        self.col = color;
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)));
    }

    fn println(&mut self, msg: String){
        writeln!(&mut self.stream, "{}", msg);
    }

    fn print(&mut self, msg: String){
        write!(&mut self.stream, "{}", msg);
    }

    fn println_color(&mut self, msg: String, color: Color){
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)));
        writeln!(&mut self.stream, "{}", msg);
        self.stream.set_color(ColorSpec::new().set_fg(Some(self.col)));
    }

    fn print_color(&mut self, msg: String, color: Color){
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)));
        write!(&mut self.stream, "{}", msg);
        self.stream.set_color(ColorSpec::new().set_fg(Some(self.col)));
    }
}

fn read_inp() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error :reading line.");
    return inp.trim().to_string();
}

fn prompt(printer: &mut Printer, msg : String) -> String {
    printer.print_color(msg, Color::Cyan);
    printer.stream.flush();
    return read_inp();
}

fn receive_command(printer: &mut Printer) {
    loop{
        let x = prompt(printer, "cmd > ".to_string());
        match x.as_ref() {
            "q" => {break;},
            "quit " => {break;},
            _ => println!("Command [\"{}\"] not found!", x)
        }
    }
}

fn main() {
    let mut printer = Printer {
        stream: StandardStream::stdout(ColorChoice::Auto),
        col: Color::White,
    };
    receive_command(&mut printer);
}
