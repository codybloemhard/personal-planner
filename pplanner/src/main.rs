extern crate termcolor;
extern crate chrono;

mod conz;
mod parser;
mod data;
mod astr;
mod save;

fn main() {
    let mut saved: Vec<u8> = Vec::new();
    save::buffer_append_u32(&mut saved, 7282);
    save::buffer_append_u32(&mut saved, 25);
    save::buffer_append_u32(&mut saved, 27827);

    save::buffer_write_file("test.save", &saved);
    let opened = save::buffer_read_file("test.save").unwrap();

    let mut iter: u32 = 0;
    let a = save::buffer_read_u32(&opened, &mut iter).1;
    let b = save::buffer_read_u32(&opened, &mut iter).1;
    let c = save::buffer_read_u32(&opened, &mut iter).1;
    println!("{},{},{}",a , b, c);

    let printer = conz::Printer::new();
    let mut parser = parser::Parser::new(printer);
    parser.start_loop();
}
