use chrono::prelude::*;
use termcolor::{ Color };

use super::conz;

pub fn now(printer : &mut conz::Printer) {
    printer.set_color(Color::Magenta);
    let dt = Local::now();
    println!("{}", dt);
}