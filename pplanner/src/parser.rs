use std::collections::HashMap;

use termcolor::{ Color };

use super::state;
use super::conz;
use super::astr;
use super::astr::AStr;

type Func = fn(&mut conz::Printer, &mut state::State, astr::AstrVec);

struct FuncTree{
    tree: HashMap<astr::Astr, Box<FuncTree>>,
    leaf: Option<Func>,
}

impl FuncTree{
    fn new() -> Box<FuncTree>{
        Box::new(
            FuncTree{
                tree: HashMap::new(),
                leaf: Option::None,
            }
        )
    }

    fn new_value(f: Func) -> Box<FuncTree>{
        Box::new(FuncTree{
            tree: HashMap::new(),
            leaf: Option::Some(f),
        })
    }

    fn push(&mut self, key: &astr::AstrVec, f: Func){
        fn _push(root: &mut FuncTree, key: &astr::AstrVec, index: usize, f: Func){
            if index >= key.len() {return;}
            let last = index == key.len() - 1;
            let res = root.tree.get_mut(&key[index]);
            match res{
                Option::None =>{
                    if last{
                        root.tree.insert(key[index].copy_from_ref(), FuncTree::new_value(f));
                    }else{
                        let mut subtree = FuncTree::new();
                        _push(&mut subtree, key, index + 1, f);
                        root.tree.insert(key[index].copy_from_ref(), subtree);
                    }
                }
                Option::Some(x) =>{
                    if last{
                        if x.leaf.is_none(){
                            x.leaf.get_or_insert(f);
                        }else{
                            panic!("FuncTree: double element");
                        }
                    }else{
                        _push(x, key, index, f);
                    }
                }
            }
        }
        _push(self, key, 0, f);
    }

    fn find(&mut self, key: &astr::AstrVec) -> Result<Func,()>{
        fn _find(root: &mut FuncTree, key: &astr::AstrVec, index: usize) -> Option<Func>{
            if index >= key.len() {return Option::None;}
            let last = index == key.len() - 1;
            let res = root.tree.get_mut(&key[index]);
            if res.is_none(){return Option::None;}
            if last{
                return res.unwrap().leaf;
            }else{
                return _find(&mut res.unwrap(), key, index + 1);
            }
        }
        let opt = _find(self, key, 0);
        if opt.is_none() {return Err(());}
        else {return Ok(opt.unwrap());}
    }
}

pub struct Parser{
    ftree: Box<FuncTree>,
    printer: conz::Printer,
    state: state::State,
}

impl Parser {
    pub fn new(printer: conz::Printer, state: state::State) -> Parser {
        let mut ftree = FuncTree::new();
        ftree.push(&astr::from_str("now").split_str(&astr::astr_whitespace()), commands::now);
        ftree.push(&astr::from_str("add deadline").split_str(&astr::astr_whitespace()), commands::add_deadline);

        return Parser {
            ftree,
            printer,
            state,
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
        let search = self.ftree.find(&command);
        match search {
            Err(_) => return false,
            Ok(x) => x(&mut self.printer, &mut self.state, command),
        }
        return true;
    }
}

mod commands {
    use super::super::conz;
    use super::super::data;
    use super::super::astr;
    use super::super::wizard;
    use super::super::state;

    pub fn now(printer: &mut conz::Printer, _: &mut state::State, _: astr::AstrVec){
        let dt = data::DT::new();
        printer.println_type(dt.str_datetime().as_ref(), conz::MsgType::Value);
    }

    pub fn add_deadline(printer: &mut conz::Printer, state: &mut state::State, _: astr::AstrVec){
        let mut fields = wizard::FieldVec::new();
        fields.add(wizard::InputType::Text, astr::from_str("title: "), true);
        fields.add(wizard::InputType::DateTime, astr::from_str("deadline: "), true);
        let res = fields.execute(printer);
        if res.is_err() { return; }
        let mut res = res.unwrap();
        let deadline = res.extract_deadline();
        if deadline.is_err() {
            printer.println_type("Error: could build deadline.", conz::MsgType::Error);
            return;
        }
        if !state.deadlines.add_deadline(deadline.unwrap()) {
            printer.println_type("Error: Could not add deadline.", conz::MsgType::Error);
            return;
        }
        if !state.deadlines.write() {
            printer.println_type("Error: Could not write deadline.", conz::MsgType::Error);
        }
        printer.println_type("Success: deadline saved", conz::MsgType::Highlight);
    }
}
