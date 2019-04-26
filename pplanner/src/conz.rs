use std::io;
use std::io::Write; //flush stdout
use std::collections::HashMap;

use termcolor::{ Color, ColorChoice, ColorSpec, StandardStream, WriteColor };
use std::sync::Mutex;

use super::astr;

pub enum MsgType {
    Normal,
    Error,
    Prompt,
    Highlight,
    Value,
}

lazy_static! {
    static ref PRINTER: Mutex<Printer> = Mutex::new(Printer::new());
}

pub fn printer() -> std::sync::MutexGuard<'static, Printer>{
    return PRINTER.lock().unwrap();
}

#[macro_export]
macro_rules! pprint{
    ($msg:expr) => {
        {conz::printer().print($msg);}
    };
}
#[macro_export]
macro_rules! pprint_color{
    ($msg:expr,$col:expr) => {
        {conz::printer().print_color($msg,$col);}
    };
}
#[macro_export]
macro_rules! pprint_type{
    ($msg:expr,$typ:expr) => {
        {conz::printer().print_type($msg,$typ);}
    };
}
#[macro_export]
macro_rules! pprint_error{
    ($pre:expr,$mid:expr,$pos:expr) => {
        {conz::printer().print_error($pre, $mid, $pos)();}
    };
}
#[macro_export]
macro_rules! pprintln{
    ($msg:expr) => {
        {conz::printer().println($msg);}
    };
}
#[macro_export]
macro_rules! pprintln_color{
    ($msg:expr,$col:expr) => {
        {conz::printer().println_color($msg,$col);}
    };
}
#[macro_export]
macro_rules! pprintln_type{
    ($msg:expr,$typ:expr) => {
        {conz::printer().println_type($msg,$typ);}
    };
}
#[macro_export]
macro_rules! pprintln_error{
    ($pre:expr,$mid:expr,$pos:expr) => {
        {conz::printer().println_error($pre, $mid, $pos);}
    };
}

pub trait Printable{
    fn print(&self);
}

pub struct Printer{
    pub stream: StandardStream,
    pub col_map: HashMap<u32, Color>,
    pub col: Color,
}

pub trait PrinterFunctions<T>{
    fn print(&mut self, msg: &T);
    fn print_color(&mut self, msg: &T, color: Color);
    fn print_type(&mut self, msg: &T, msgtype: MsgType);
    fn print_error(&mut self, prefix: &T, middle: &T, postfix: &T);
    fn println(&mut self, msg: &T);
    fn println_color(&mut self, msg: &T, color: Color);
    fn println_type(&mut self, msg: &T, msgtype: MsgType);
    fn println_error(&mut self, prefix: &T, middle: &T, postfix: &T);
}

impl Printer {
    pub fn new() -> Printer {
        let mut color_map: HashMap<u32, Color> = HashMap::new();
        let normal_color = Color::Green;
        color_map.insert(MsgType::Normal as u32, normal_color);
        color_map.insert(MsgType::Error as u32, Color::Red);
        color_map.insert(MsgType::Prompt as u32, Color::Cyan);
        color_map.insert(MsgType::Highlight as u32, Color::White);
        color_map.insert(MsgType::Value as u32, Color::Yellow);
        return Printer {
            stream: StandardStream::stdout(ColorChoice::Always),
            col_map: color_map,
            col: normal_color,
        }
    }

    fn _get_color(&mut self, msgtype: MsgType) -> Color{
        match self.col_map.get(&(msgtype as u32)){
            None => return self.col,
            Some(x) => return *x,
        }
    }

    fn _set_color(&mut self, color: Color){
        self.stream.set_color(ColorSpec::new().set_fg(Some(color)))
            .expect("Error: Printer > set_color > 0");
    }
}

impl<T: astr::TOSTRING> PrinterFunctions<T> for Printer{
    fn print(&mut self, msg: &T){
        write!(&mut self.stream, "{}", msg.tostring())
            .expect("Error: Printer > print > 0");
    }

    fn print_color(&mut self, msg: &T, color: Color){
        self._set_color(color);
        self.print(msg);
        self._set_color(self.col);
    }

    fn print_type(&mut self, msg: &T, msgtype: MsgType){
        let col = self._get_color(msgtype);
        self.print_color(msg, col);
    }

    fn print_error(&mut self, prefix: &T, middle: &T, postfix: &T){
        let prec = self._get_color(MsgType::Error);
        let midc = self._get_color(MsgType::Highlight);
        let posc = self._get_color(MsgType::Error);
        self.print_color(prefix, prec);
        self.print_color(middle, midc);
        self.print_color(postfix, posc);
    }

    fn println(&mut self, msg: &T){
        self.print(msg);
        self.print(&"\n");
    }

    fn println_color(&mut self, msg: &T, color: Color){
        self.print_color(msg, color);
        self.print(&"\n");
    }

    fn println_type(&mut self, msg: &T, msgtype: MsgType){
        self.print_type(msg, msgtype);
        self.print(&"\n");
    }

    fn println_error(&mut self, prefix: &T, middle: &T, postfix: &T){
        self.print_error(prefix, middle, postfix);
        self.print(&"\n");
    }
}

pub fn read_inp() -> String{
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error :reading line.");
    return inp.trim().to_string();
}

pub fn prompt(msg : &str) -> String{
    {printer().print_color(&msg, Color::Cyan);}
    {printer().stream.flush()
        .expect("Error: Printer > println_color > 0");}
    return read_inp();
}

pub fn read_bool(msg: &str) -> bool{
    let line = prompt(&msg);
    match line.as_ref(){
        "y" => true,
        "ye" => true,
        "yes" => true,
        "ok" => true,
        "+" => true,
        _ => false,
    }
}