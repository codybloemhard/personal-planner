use std::collections::HashMap;
use std::mem;
use std::cell::RefCell;
use std::rc::Rc;

use termcolor::{ Color };

use super::conz;
use super::astr;
use super::astr::AStr;

type Func = fn(&mut conz::Printer, astr::AstrVec);

struct Node{
    tree: HashMap<astr::Astr, Box<Node>>,
    leaf: Option<Func>,
}

fn new_node() -> Node{
    Node{
        tree: HashMap::new(),
        leaf: Option::None,
    }
}

fn new_node_value(f: Func) -> Box<Node>{
    Box::new(Node{
        tree: HashMap::new(),
        leaf: Option::Some(f),
    })
}

fn push(root: &mut Node, key: &astr::AstrVec, f: Func){
    fn _push(root: &mut Node, key: &astr::AstrVec, index: usize, f: Func){
        if index >= key.len() {return;}
        let last = index == key.len() - 1;
        let res = root.tree.get_mut(&key[index]);
        if res.is_none(){
            if last{
                root.tree.insert(key[index].copy_from_ref(), new_node_value(f));
            }else{
                let mut subtree = Box::new(new_node());
                _push(&mut subtree, key, index + 1, f);
                root.tree.insert(key[index].copy_from_ref(), subtree);
            }
        }else{
            let x = res.unwrap();
            if last{
                if x.leaf.is_none(){
                    x.leaf.get_or_insert(f);
                }else{
                    panic!("oh no");
                }
            }else{
                _push(x, key, index, f);
            }
        }
    }
    _push(root, key, 0, f);
}

fn find(root: &mut Node, key: &astr::AstrVec) -> Result<Func,()>{
    fn _find(root: &mut Node, key: &astr::AstrVec, index: usize) -> Option<Func>{
        if index >= key.len() {return Option::None;}
        let last = index == key.len() - 1;
        let res = root.tree.get_mut(&key[index]);
        if res.is_none(){return Option::None;}
        else{
            if last{
                return res.unwrap().leaf;
            }else{
                return _find(&mut res.unwrap(), key, index + 1);
            }
        }
    }
    let opt = _find(root, key, 0);
    if opt.is_none() {return Err(());}
    else {return Ok(opt.unwrap());}
}

pub struct Parser{
    ftree: Box<Node>,
    printer: conz::Printer,
}

impl Parser {
    pub fn new(printer: conz::Printer) -> Parser {
        let mut ftree = Box::new(new_node());
        push(&mut ftree, &astr::from_str("now").split_str(&astr::astr_whitespace()), commands::now);
        push(&mut ftree, &astr::from_str("add deadline").split_str(&astr::astr_whitespace()), commands::add_deadline);

        return Parser {
            ftree,
            printer,
        }
    }

    pub fn start_loop(&mut self) {
        self.printer.println_type("Henlo Fren!", conz::MsgType::Prompt);
        self.printer.println_type("pplanner: a ascii cli time management tool.", conz::MsgType::Prompt);
        self.printer.println_type("Made by Cody Bloemhard.", conz::MsgType::Prompt);
        loop{
            let x = conz::prompt(&mut self.printer, "cmd > ");
            let y = x.as_ref();
            match y {
                "q" => break,
                "quit" => break,
                _ => {
                    let found_cmd = self.parse_and_run(y);
                    if found_cmd { continue; }
                    self.printer.println_error("Error: Command not found: \"", y, "\"!");
                }
            }
        }
        self.printer.println_color("Bye!", Color::Cyan);
    }

    fn parse_and_run(&mut self, line: &str) -> bool{
        let command = astr::from_str(line).split_str(&astr::astr_whitespace());
        let search = find(&mut self.ftree, &command);
        match search {
            Err(_) => return false,
            Ok(x) => x(&mut self.printer, command),
        }
        return true;
    }
}

mod commands {
    use super::super::conz;
    use super::super::data;
    use super::super::astr;
    use super::super::wizard;

    pub fn dummy(_printer: &mut conz::Printer, _command: astr::AstrVec){ }

    pub fn now(printer: &mut conz::Printer, _command: astr::AstrVec){
        let dt = data::DT::new();
        printer.println_type(dt.str_datetime().as_ref(), conz::MsgType::Value);
    }

    pub fn add_deadline(printer: &mut conz::Printer, _command: astr::AstrVec){
        let mut fields: Vec<wizard::Field> = Vec::new();
        fields.push(wizard::make_field(wizard::InputType::Text, astr::from_str("title: "), true));
        fields.push(wizard::make_field(wizard::InputType::DateTime, astr::from_str("deadline: "), true));
        let res = wizard::execute(&fields, printer);
        if res.is_err() { return; }
        let res = res.unwrap();
        
    }
}
