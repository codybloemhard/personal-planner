use std::io;
use std::io::Write; //flush stdout
use std::collections::VecDeque;

use super::astr;
use super::astr::{TOSTRING};

#[derive(Clone)]
pub enum MsgType {
    Normal,
    Error,
    Prompt,
    Highlight,
    Value,
}

pub trait Printable{
    fn print(&self);
}

pub trait PrettyPrintable{
    type ArgType;
    fn pretty_print(&self, arg: &Self::ArgType) -> (astr::AstrVec,Vec<MsgType>);
    fn lengths(arg: &Self::ArgType) -> Vec<u16>;
    fn titles(arg: &Self::ArgType) -> Vec<astr::Astr>;
}

fn set_colour(msgtype: MsgType){
    let colorcode = match msgtype {
        MsgType::Normal => "\x1B[32m",
        MsgType::Error => "\x1B[31m",
        MsgType::Prompt => "\x1B[36m",
        MsgType::Highlight => "\x1B[37m",
        MsgType::Value => "\x1B[33m",
    };
    print!("{}", colorcode);
}

pub fn print<T: astr::TOSTRING>(msg: T){
    set_colour(MsgType::Normal);
    print!("{}", msg.tostring());
}

pub fn print_type<T: astr::TOSTRING>(msg: T, msgtype: MsgType){
    set_colour(msgtype);
    print!("{}", msg.tostring());
}

pub fn print_error<T: astr::TOSTRING>(pre: T, mid: T, pos: T){
    set_colour(MsgType::Error);
    print!("{}", pre.tostring());
    set_colour(MsgType::Highlight);
    print!("{}", mid.tostring());
    set_colour(MsgType::Error);
    print!("{}", pos.tostring());
}

pub fn println<T: astr::TOSTRING>(msg: T){
    set_colour(MsgType::Normal);
    println!("{}", msg.tostring());
}

pub fn println_type<T: astr::TOSTRING>(msg: T, msgtype: MsgType){
    set_colour(msgtype);
    println!("{}", msg.tostring());
}

pub fn println_error<T: astr::TOSTRING>(pre: T, mid: T, pos: T){
    set_colour(MsgType::Error);
    print!("{}", pre.tostring());
    set_colour(MsgType::Highlight);
    print!("{}", mid.tostring());
    set_colour(MsgType::Error);
    println!("{}", pos.tostring());
}

pub fn read_inp() -> String{
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error :reading line.");
    return inp.trim().to_string();
}

pub fn prompt(msg : &str) -> String{
    print_type(msg, MsgType::Prompt);
    /*{printer().stream.flush()
        .expect("Error: Printer > println_color > 0");}*/
    std::io::stdout().flush().expect("Error: stdout flush failed.");
    return read_inp();
}

pub fn read_bool(msg: &str, inputs: &mut Option<VecDeque<astr::Astr>>) -> bool{
    let line;
    if inputs.is_none(){line = prompt(&msg);}
    else{
        let res = inputs.as_mut().unwrap().pop_front();
        if res.is_none(){line = prompt(&msg);}
        else {line = res.unwrap().tostring();}
    }
    match line.as_ref(){
        "y" => true,
        "ye" => true,
        "yes" => true,
        "ok" => true,
        "+" => true,
        _ => false,
    }
}
