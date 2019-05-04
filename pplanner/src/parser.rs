use std::collections::HashSet;
use std::collections::HashMap;

use termcolor::{ Color };

use super::state;
use super::conz;
use super::conz::PrinterFunctions;
use super::astr;
use super::astr::{AStr};
use super::commands;

type Func = fn(&mut state::State, astr::AstrVec);

pub struct FuncTree{
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
                        _push(x, key, index + 1, f);
                    }
                }
            }
        }
        _push(self, key, 0, f);
    }

    fn find(&mut self, key: &astr::AstrVec) -> Option<Func>{
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
        return _find(self, key, 0);
    }
}

pub struct Parser{
    ftree: Box<FuncTree>,
    state: state::State,
}

impl Parser {
    pub fn new(mut state: state::State) -> Parser {
        let mut ftree = FuncTree::new();
        let mut fset = HashSet::new();
        Parser::add("now", commands::now, &mut ftree, &mut fset);
        Parser::add("help", commands::help, &mut ftree, &mut fset);
        Parser::add("ls commands", commands::ls_commands, &mut ftree, &mut fset);
        Parser::add("mk point", commands::mk_point, &mut ftree, &mut fset);
        Parser::add("ls points", commands::ls_points, &mut ftree, &mut fset);
        Parser::add("ls points archive", commands::ls_points_archive, &mut ftree, &mut fset);
        Parser::add("inspect point", commands::inspect_point, &mut ftree, &mut fset);
        Parser::add("rm point", commands::rm_point, &mut ftree, &mut fset);
        Parser::add("clean points", commands::clean_points, &mut ftree, &mut fset);
        Parser::add("edit point", commands::edit_point, &mut ftree, &mut fset);
        Parser::add("mk todo", commands::mk_todo, &mut ftree, &mut fset);
        Parser::add("ls todos", commands::ls_todos, &mut ftree, &mut fset);
        Parser::add("ls todos archive", commands::ls_todos_archive, &mut ftree, &mut fset);
        Parser::add("rm todo", commands::rm_todos, &mut ftree, &mut fset);
        Parser::add("flush files", commands::flush_files, &mut ftree, &mut fset);

        state.fset = fset;
        return Parser {
            ftree,
            state,
        }
    }

    fn add(name: &str, func: Func, ftree: &mut Box<FuncTree>, fset: &mut HashSet<astr::Astr>){
        let splitted = astr::from_str(name).split_str(&astr::astr_whitespace());
        ftree.push(&splitted, func);
        fset.insert(astr::from_str(name));
    }

    fn do_quit(&self) -> bool{
        if self.state.is_clean() {return true;}
        pprintln_type!(&"Unsaved files! Do you really want to quit?\nYou can say no and try \"flush files\"", conz::MsgType::Highlight);
        let x = conz::prompt("Quit? y/*: ");
        match x.as_ref(){
            "y" => return true,
            _ => return false,
        }
    }

    fn extract_args(line: astr::Astr) -> (astr::Astr, astr::Astr){
        let mut mode = 0;
        let mut command = astr::new();
        let mut args = astr::new();
        for ch in line{
            if ch == '(' as u8{
                mode = 1;
            }else if ch == ')' as u8{
                break;
            }else if mode == 0{
                command.push(ch);
            }else if mode == 1{
                args.push(ch);
            }
        }
        return (command,args);
    }

    pub fn start_loop(&mut self) {
        pprintln_type!(&"Henlo Fren!", conz::MsgType::Prompt);
        pprintln_type!(&"pplanner: a ascii cli time management tool.", conz::MsgType::Prompt);
        pprintln_type!(&"Made by Cody Bloemhard.", conz::MsgType::Prompt);
        loop{
            let x = conz::prompt("cmd > ");
            let y = x.as_ref();
            match y{
                "q" => if self.do_quit() {break;},
                "quit" => if self.do_quit() {break;},
                _ => {
                    let (com,arg) = Parser::extract_args(astr::from_str(y));
                    let found_cmd = self.parse_and_run(com, arg);
                    if found_cmd { continue; }
                    pprintln_error!(&"Fail: Command not found: \"", &y, &"\"!");
                }
            }
        }
        pprintln_color!(&"Bye!", Color::Cyan);
    }

    fn parse_and_run(&mut self, command: astr::Astr, args: astr::Astr) -> bool{
        let command = command.split_str(&astr::astr_whitespace());
        let args = args.split_str(&astr::from_str(","));
        let search = self.ftree.find(&command);
        match search {
            Option::None => return false,
            Option::Some(x) => x(&mut self.state, args),
        }
        return true;
    }
}
