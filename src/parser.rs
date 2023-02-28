use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

use super::state;
use super::conz;
use super::astr;
use super::astr::{Astr,AStr,ToAstr,AstrsRef};
use super::commands;

type Func = fn(&mut state::State, astr::AstrVec, Option<VecDeque<astr::Astr>>);

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

    fn push(&mut self, key: AstrsRef, f: Func){
        fn _push(root: &mut FuncTree, key: AstrsRef, index: usize, f: Func){
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

    fn find(&mut self, key: AstrsRef) -> Option<Func>{
        fn _find(root: &mut FuncTree, key: AstrsRef, index: usize) -> Option<Func>{
            if index >= key.len() {return Option::None;}
            let last = index == key.len() - 1;
            let res = root.tree.get_mut(&key[index]);
            res.as_ref()?;
            if last{
                res.unwrap().leaf
            }else{
                _find(res.unwrap(), key, index + 1)
            }
        }
        _find(self, key, 0)
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
        Parser::add("license", commands::license, &mut ftree, &mut fset);
        Parser::add("ls commands", commands::ls_commands, &mut ftree, &mut fset);
        Parser::add("ls days", commands::ls_days, &mut ftree, &mut fset);
        Parser::add("ls months", commands::ls_months, &mut ftree, &mut fset);

        Parser::add("mk point", commands::mk_point, &mut ftree, &mut fset);
        Parser::add("ls points", commands::ls_points, &mut ftree, &mut fset);
        Parser::add("ls points archive", commands::ls_points_archive, &mut ftree, &mut fset);
        Parser::add("inspect point", commands::inspect_point, &mut ftree, &mut fset);
        Parser::add("rm points", commands::rm_points, &mut ftree, &mut fset);
        Parser::add("clean points", commands::clean_points, &mut ftree, &mut fset);
        Parser::add("edit points", commands::edit_points, &mut ftree, &mut fset);

        Parser::add("mk plan", commands::mk_plan, &mut ftree, &mut fset);
        Parser::add("ls plans", commands::ls_plans, &mut ftree, &mut fset);
        Parser::add("ls plans archive", commands::ls_plans_archive, &mut ftree, &mut fset);
        Parser::add("rm plans", commands::rm_plans, &mut ftree, &mut fset);
        Parser::add("edit plans", commands::edit_plans, &mut ftree, &mut fset);
        Parser::add("mv plans", commands::mv_plans, &mut ftree, &mut fset);

        Parser::add("mk slice", commands::mk_slice, &mut ftree, &mut fset);
        Parser::add("ls slices", commands::ls_slices, &mut ftree, &mut fset);
        Parser::add("ls slices archive", commands::ls_slices_archive, &mut ftree, &mut fset);
        Parser::add("inspect slice", commands::inspect_slice, &mut ftree, &mut fset);
        Parser::add("rm slices", commands::rm_slices, &mut ftree, &mut fset);
        Parser::add("clean slices", commands::clean_slices, &mut ftree, &mut fset);
        Parser::add("edit slices", commands::edit_slices, &mut ftree, &mut fset);

        Parser::add("mk todo", commands::mk_todo, &mut ftree, &mut fset);
        Parser::add("ls todos", commands::ls_todos, &mut ftree, &mut fset);
        Parser::add("ls todos archive", commands::ls_todos_archive, &mut ftree, &mut fset);
        Parser::add("rm todos", commands::rm_todos, &mut ftree, &mut fset);
        Parser::add("clean todos", commands::clean_todos, &mut ftree, &mut fset);
        Parser::add("tick todos", commands::tick_todos, &mut ftree, &mut fset);

        Parser::add("status", commands::status, &mut ftree, &mut fset);
        Parser::add("flush files", commands::flush_files, &mut ftree, &mut fset);
        Parser::add("_test_keys", commands::test_keys, &mut ftree, &mut fset);
        Parser::add("_missing_help", commands::missing_help, &mut ftree, &mut fset);
        state.fset = fset;
        Parser {
            ftree,
            state,
        }
    }

    fn add(name: &str, func: Func, ftree: &mut FuncTree, fset: &mut HashSet<astr::Astr>){
        let splitted = astr::from_str(name).split_str(&astr::astr_whitespace());
        ftree.push(&splitted, func);
        fset.insert(astr::from_str(name));
    }

    fn do_quit(&self) -> bool{
        if self.state.is_clean() {return true;}
        conz::println_type("Unsaved files! Do you really want to quit?\nYou can say no and try \"flush files\"", conz::MsgType::Highlight);
        let x = conz::prompt("Quit? y/*: ");
        matches!(x.as_ref(), "y")
    }

    fn extract_args(line: astr::Astr) -> (astr::Astr, astr::Astr){
        let mut mode = 0;
        let mut command = Astr::new();
        let mut args = Astr::new();
        for ch in line.0{
            if ch == b'('{
                mode = 1;
            }else if ch == b')'{
                break;
            }else if mode == 0{
                command.0.push(ch);
            }else if mode == 1{
                args.0.push(ch);
            }
        }
        (command,args)
    }

    pub fn parse_and_run(&mut self, rawstr: &str, inputs: Option<VecDeque<astr::Astr>>) -> bool{
        let (com,arg) = Parser::extract_args(astr::from_str(rawstr));
        let command = com.split_str(&astr::astr_whitespace());
        let args = arg.split_str(&astr::from_str(","));
        let search = self.ftree.find(&command);
        match search {
            Option::None => {
                conz::println_error("Fail: Command not found: \"", rawstr, "\"!");
                let words = astr::from_str(rawstr).split_str(&astr::astr_whitespace());
                let mut maxcount = 0.0;
                let mut best = Vec::new();
                for f in &self.state.fset{
                    let mut count = 0.0;
                    let splitted = f.split_str(&astr::astr_whitespace());
                    for w in &words{
                        for s in &splitted{
                            if w == s {
                                count += 4.0;
                            }else{
                                count += w.sameness(s);
                            }
                        }
                    }
                    count /= (words.len() * splitted.len()) as f32;
                    if count > maxcount + 1.0{
                        best.clear();
                        best.push(f.clone());
                        maxcount = count;
                    }else if count >= maxcount{
                        best.push(f.clone());
                    }
                }
                if !best.is_empty(){
                    conz::println_type("Best matches: ", conz::MsgType::Normal);
                    for b in best{
                        conz::println_type(b.disp(), conz::MsgType::Highlight);
                    }
                }
                return false;
            },
            Option::Some(x) => x(&mut self.state, args, inputs),
        }
        true
    }
}

pub fn process_cli_args(args: Vec<String>, parser: &mut Parser){
    let mut phase = 0;
    let mut command = String::new();
    let mut arguments = VecDeque::new();
    for arg in args.iter().skip(1){
        if arg == "help" || arg == "--help"{
            commands::help_cli();
        }
        else if arg == "-i"{
            phase = 1;
        }
        else if phase == 0{
            command.push_str(arg);
            command.push(' ');
        } else {
            phase = 2;
            let splitted = arg.to_astr().split_str(&astr::from_str(","));
            for s in splitted{
                arguments.push_back(s);
            }
        }
    }
    let arguments = if phase == 2{
        Some(arguments)
    } else {
        None
    };
    parser.parse_and_run(&command, arguments);
    parser.do_quit();
}
