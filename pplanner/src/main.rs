extern crate termcolor;

use termcolor::{ Color };

mod conz;

fn receive_command(printer: &mut conz::Printer) {
    loop{
        let x = conz::prompt(printer, String::from("cmd > "));
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
    let mut printer = conz::create_printer();
    printer.set_color(Color::Green);
    receive_command(&mut printer);
}
