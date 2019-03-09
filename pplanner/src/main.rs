extern crate termcolor;
extern crate chrono;

mod conz;
mod parser;
mod data;
mod astr;
mod save;
use save::Binairizable;

fn main() {
    let mut saved: Vec<u8> = Vec::new();
    save::buffer_append_u32(&mut saved, 7282);
    save::buffer_append_u32(&mut saved, 25);
    save::buffer_append_string(&mut saved, &astr::from_str("henlo frens!"),);
    save::buffer_append_u32(&mut saved, 27827);
    let dt0 = data::DT::new();
    save::buffer_append_buffer(&mut saved, &dt0.to_binairy());

    save::buffer_write_file("test.save", &saved);
    let opened = save::buffer_read_file("test.save").unwrap();
    let mut iter: u32 = 0;
    let a = save::buffer_read_u32(&opened, &mut iter).unwrap();
    let b = save::buffer_read_u32(&opened, &mut iter).unwrap();
    let c = save::buffer_read_string(&opened, &mut iter).unwrap();
    let d = save::buffer_read_u32(&opened, &mut iter).unwrap();
    let e = data::DT::from_binairy(&opened, &mut iter).unwrap();
    println!("{},{},{},{},{}", a, b, astr::to_string(&c), d, e.str_datetime());
    println!("{}", save::buffer_read_string(&opened, &mut iter).is_ok());

    let printer = conz::Printer::new();
    let mut parser = parser::Parser::new(printer);
    parser.start_loop();
}
