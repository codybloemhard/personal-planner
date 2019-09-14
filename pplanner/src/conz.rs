use std::io::Write; //flush stdout
use std::collections::VecDeque;
use term_basics_linux::tbl;

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
        MsgType::Normal => tbl::UserColour::Green,
        MsgType::Error => tbl::UserColour::Red,
        MsgType::Prompt => tbl::UserColour::Cyan,
        MsgType::Highlight => tbl::UserColour::Grey,
        MsgType::Value => tbl::UserColour::Yellow,
    };
    tbl::set_colour(colorcode, tbl::FGBG::FG);
    let typecode = match msgtype {
        MsgType::Normal => tbl::TextStyle::Std,
        MsgType::Error => tbl::TextStyle::Underlined,
        MsgType::Prompt => tbl::TextStyle::Bold,
        MsgType::Highlight => tbl::TextStyle::Italic,
        MsgType::Value => tbl::TextStyle::Bold,
    };
    tbl::set_style(typecode);
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

pub fn prompt(msg : &str) -> String{
    print_type(msg, MsgType::Prompt);
    std::io::stdout().flush().expect("Error: stdout flush failed.");
    set_colour(MsgType::Normal);
    return tbl::input_field();
}

pub fn read_bool(msg: &str, inputs: &mut Option<VecDeque<astr::Astr>>) -> bool{
    let line;
    if inputs.is_none(){line = prompt(&msg);}
    else{
        let res = inputs.as_mut().unwrap().pop_front();
        if res.is_none(){line = prompt(&msg);}
        else {line = res.unwrap().tostring();}
    }
    return tbl::string_to_bool(&line);
}
