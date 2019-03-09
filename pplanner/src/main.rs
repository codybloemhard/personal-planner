extern crate termcolor;
extern crate chrono;

mod conz;
mod parser;
mod data;
mod astr;
mod save;
use save::Bufferable;

fn main() {
    let mut saved: Vec<u8> = Vec::new();
    u32::into_buffer(&7282, &mut saved);
    u32::into_buffer(&25, &mut saved);
    save::buffer_append_string(&mut saved, &astr::from_str("henlo frens!"),);
    u32::into_buffer(&6666, &mut saved);
    let dt0 = data::DT::new();
    dt0.into_buffer(&mut saved);

    save::buffer_write_file("test.save", &saved);
    let opened = save::buffer_read_file("test.save").unwrap();
    let mut iter: u32 = 0;
    let a = u32::from_buffer(&opened, &mut iter).unwrap();
    let b = u32::from_buffer(&opened, &mut iter).unwrap();
    let c = save::buffer_read_string(&opened, &mut iter).unwrap();
    let d = u32::from_buffer(&opened, &mut iter).unwrap();
    let e = data::DT::from_buffer(&opened, &mut iter).unwrap();
    println!("{},{},{},{},{}", a, b, astr::to_string(&c), d, e.str_datetime());
    println!("{}", save::buffer_read_string(&opened, &mut iter).is_ok());

    let printer = conz::Printer::new();
    let mut parser = parser::Parser::new(printer);
    parser.start_loop();
}
