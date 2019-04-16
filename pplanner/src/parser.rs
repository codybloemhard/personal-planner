use std::collections::HashMap;

use termcolor::{ Color };

use super::state;
use super::conz;
use super::astr;
use super::astr::AStr;

type Func = fn(&mut state::State, astr::AstrVec);

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
    state: state::State,
}

impl Parser {
    pub fn new(state: state::State) -> Parser {
        let mut ftree = FuncTree::new();
        ftree.push(&astr::from_str("now").split_str(&astr::astr_whitespace()), commands::now);
        ftree.push(&astr::from_str("mk point").split_str(&astr::astr_whitespace()), commands::mk_point);
        ftree.push(&astr::from_str("flush files").split_str(&astr::astr_whitespace()), commands::flush_files);

        return Parser {
            ftree,
            state,
        }
    }

    fn do_quit(&self) -> bool{
        if self.state.is_clean() {return true;}
        conz::printer().println_type("Unsaved files! Do you really want to quit?\nYou can say no and try \"flush files\"", conz::MsgType::Highlight);
        let x = conz::prompt("Quit? y/*: ");
        match x.as_ref(){
            "y" => return true,
            _ => return false,
        }
    }

    pub fn start_loop(&mut self) {
        conz::printer().println_type("Henlo Fren!", conz::MsgType::Prompt);
        conz::printer().println_type("pplanner: a ascii cli time management tool.", conz::MsgType::Prompt);
        conz::printer().println_type("Made by Cody Bloemhard.", conz::MsgType::Prompt);
        loop{
            let x = conz::prompt("cmd > ");
            let y = x.as_ref();
            match y {
                "q" => if self.do_quit() {break;},
                "quit" => if self.do_quit() {break;},
                _ => {
                    let found_cmd = self.parse_and_run(y);
                    if found_cmd { continue; }
                    conz::printer().println_error("Error: Command not found: \"", y, "\"!");
                }
            }
        }
        conz::printer().println_color("Bye!", Color::Cyan);
    }

    fn parse_and_run(&mut self, line: &str) -> bool{
        let command = astr::from_str(line).split_str(&astr::astr_whitespace());
        let search = self.ftree.find(&command);
        match search {
            Err(_) => return false,
            Ok(x) => x(&mut self.state, command),
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

    pub fn now(_: &mut state::State, _: astr::AstrVec){
        let dt = data::DT::new();
        conz::printer().println_type(dt.str_datetime().as_ref(), conz::MsgType::Value);
    }

    pub fn mk_point(state: &mut state::State, _: astr::AstrVec){
        let mut fields = wizard::FieldVec::new();
        fields.add(wizard::InputType::Text, astr::from_str("title: "), false);
        fields.add(wizard::InputType::Bool, astr::from_str("is deadline?: "), false);
        fields.add(wizard::InputType::DateTime, astr::from_str("time date: "), true);
        let res = fields.execute();
        if res.is_err() { return; }
        let mut res = res.unwrap();
        let point = res.extract_point();
        if point.is_err() {return;}
        state.points.add_item(point.unwrap());
        if !state.points.write() {return;}
        conz::printer().println_type("Success: Point saved.", conz::MsgType::Highlight);
    }

    pub fn flush_files(state: &mut state::State, _: astr::AstrVec){
        if state.is_clean() {
            conz::printer().println_type("All files clean, nothing to do.", conz::MsgType::Highlight);
            return;
        }
        let res = state.flush_files();
        if res {
            conz::printer().println_type("Success: Flushed all dirty files.", conz::MsgType::Highlight);
        }else{
            conz::printer().println_type("Error: Could not flush all dirty files.", conz::MsgType::Error);
        }
    }
}
