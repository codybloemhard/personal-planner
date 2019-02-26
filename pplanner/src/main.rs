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
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)))
            .expect("Error: Printer > set_color > 0");
    }

    fn println(&mut self, msg: String){
        writeln!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > println > 0");
    }

    fn print(&mut self, msg: String){
        write!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > print > 0");
    }

    fn println_color(&mut self, msg: String, color: Color){
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)))
            .expect("Error: Printer > println_color > 0");
        writeln!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > println_color > 1");
        self.stream.set_color(ColorSpec::new().set_fg(Some(self.col)))
            .expect("Error: Printer > println_color > 2");
    }

    fn print_color(&mut self, msg: String, color: Color){
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)))
            .expect("Error: Printer > print_color > 0");
        write!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > print_color > 1");
        self.stream.set_color(ColorSpec::new().set_fg(Some(self.col)))
            .expect("Error: Printer > print_color > 2");
    }
}

fn read_inp() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error :reading line.");
    return inp.trim().to_string();
}

fn prompt(printer: &mut Printer, msg : String) -> String {
    printer.print_color(msg, Color::Cyan);
    printer.stream.flush()
        .expect("Error: Printer > println_color > 0");
    return read_inp();
}

fn receive_command(printer: &mut Printer) {
    loop{
        let x = prompt(printer, String::from("cmd > "));
        match x.as_ref() {
            "q" => break,
            "quit" => break,
            _ => {
                printer.print_color(String::from("Error: Command not found: \""), Color::Red);
                printer.print_color(x, Color::Green);
                printer.println_color(String::from("\"!"), Color::Red);
            }
        }
    }
    printer.println_color(String::from("Bye!"), Color::Cyan);
}

fn main() {
    let mut printer = Printer {
        stream: StandardStream::stdout(ColorChoice::Auto),
        col: Color::White,
    };
    printer.set_color(Color::Green);
    receive_command(&mut printer);
}
