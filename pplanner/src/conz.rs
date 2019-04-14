use std::io;
use std::io::Write; //flush stdout
use std::collections::HashMap;

use termcolor::{ Color, ColorChoice, ColorSpec, StandardStream, WriteColor };
use std::sync::Mutex;

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

pub struct Printer{
    stream: StandardStream,
    col_map: HashMap<u32, Color>,
    col: Color,
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

    pub fn print(&mut self, msg: &str){
        write!(&mut self.stream, "{}", msg)
            .expect("Error: Printer > print > 0");
    }

    pub fn print_color(&mut self, msg: &str, color: Color){
        self._set_color(color);
        self.print(msg);
        self._set_color(self.col);
    }

    pub fn print_type(&mut self, msg: &str, msgtype: MsgType){
        let col = self._get_color(msgtype);
        self.print_color(msg, col);
    }

    pub fn print_error(&mut self, prefix: &str, middle: &str, postfix: &str){
        let prec = self._get_color(MsgType::Error);
        let midc = self._get_color(MsgType::Highlight);
        let posc = self._get_color(MsgType::Error);
        self.print_color(prefix, prec);
        self.print_color(middle, midc);
        self.print_color(postfix, posc);
    }

    pub fn println(&mut self, msg: &str){
        self.print(msg);
        self.print("\n");
    }

    pub fn println_color(&mut self, msg: &str, color: Color){
        self.print_color(msg, color);
        self.print("\n");
    }

    pub fn println_type(&mut self, msg: &str, msgtype: MsgType){
        self.print_type(msg, msgtype);
        self.print("\n");
    }

    pub fn println_error(&mut self, prefix: &str, middle: &str, postfix: &str){
        self.print_error(prefix, middle, postfix);
        self.print("\n");
    }
}

pub fn read_inp() -> String {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Error :reading line.");
    return inp.trim().to_string();
}

pub fn prompt(msg : &str) -> String {
    let mut printer = printer();
    printer.print_color(msg, Color::Cyan);
    printer.stream.flush()
        .expect("Error: Printer > println_color > 0");
    return read_inp();
}
