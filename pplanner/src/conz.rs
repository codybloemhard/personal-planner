use std::io;
use std::io::Write; //flush stdout

use termcolor::{ Color, ColorChoice, ColorSpec, StandardStream, WriteColor };

pub struct Printer{
    pub stream: StandardStream,
    pub col: Color,
}

impl Printer {
    pub fn new() -> Printer {
        return Printer {
            stream: StandardStream::stdout(ColorChoice::Always),
            col: Color::White,
        }
    }

    pub fn set_color(&mut self, color: Color){
        self.col = color;
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)))
            .expect("Error: Printer > set_color > 0");
    }

    pub fn println(&mut self, msg: String){
        writeln!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > println > 0");
    }

    pub fn print(&mut self, msg: String){
        write!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > print > 0");
    }

    pub fn println_color(&mut self, msg: String, color: Color){
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)))
            .expect("Error: Printer > println_color > 0");
        writeln!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > println_color > 1");
        self.stream.set_color(ColorSpec::new().set_fg(Some(self.col)))
            .expect("Error: Printer > println_color > 2");
    }

    pub fn print_color(&mut self, msg: String, color: Color){
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)))
            .expect("Error: Printer > print_color > 0");
        write!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > print_color > 1");
        self.stream.set_color(ColorSpec::new().set_fg(Some(self.col)))
            .expect("Error: Printer > print_color > 2");
    }
}

pub fn read_inp() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error :reading line.");
    return inp.trim().to_string();
}

pub fn prompt(printer: &mut Printer, msg : String) -> String {
    printer.print_color(msg, Color::Cyan);
    printer.stream.flush()
        .expect("Error: Printer > println_color > 0");
    return read_inp();
}
