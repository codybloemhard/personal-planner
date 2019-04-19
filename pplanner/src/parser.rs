use std::collections::HashMap;

use termcolor::{ Color };

use super::state;
use super::conz;
use super::conz::PrinterFunctions;
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
        ftree.push(&astr::from_str("ls points").split_str(&astr::astr_whitespace()), commands::ls_points);
        ftree.push(&astr::from_str("rm point").split_str(&astr::astr_whitespace()), commands::rm_point);
        ftree.push(&astr::from_str("flush files").split_str(&astr::astr_whitespace()), commands::flush_files);

        return Parser {
            ftree,
            state,
        }
    }

    fn do_quit(&self) -> bool{
        if self.state.is_clean() {return true;}
        conz::printer().println_type(&"Unsaved files! Do you really want to quit?\nYou can say no and try \"flush files\"", conz::MsgType::Highlight);
        let x = conz::prompt("Quit? y/*: ");
        match x.as_ref(){
            "y" => return true,
            _ => return false,
        }
    }

    pub fn start_loop(&mut self) {
        conz::printer().println_type(&"Henlo Fren!", conz::MsgType::Prompt);
        conz::printer().println_type(&"pplanner: a ascii cli time management tool.", conz::MsgType::Prompt);
        conz::printer().println_type(&"Made by Cody Bloemhard.", conz::MsgType::Prompt);
        loop{
            let x = conz::prompt("cmd > ");
            let y = x.as_ref();
            match y {
                "q" => if self.do_quit() {break;},
                "quit" => if self.do_quit() {break;},
                _ => {
                    let found_cmd = self.parse_and_run(y);
                    if found_cmd { continue; }
                    conz::printer().println_error(&"Error: Command not found: \"", &y, &"\"!");
                }
            }
        }
        conz::printer().println_color(&"Bye!", Color::Cyan);
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
    use super::super::conz::PrinterFunctions;
    use super::super::conz::Printable;
    use super::super::data;
    use super::super::astr;
    use super::super::astr::AStr;
    use super::super::astr::ToAstr;
    use super::super::wizard;
    use super::super::state;

    pub fn now(_: &mut state::State, _: astr::AstrVec){
        let dt = data::DT::new();
        conz::printer().print_type(&dt.str_datetime(), conz::MsgType::Value);
        conz::printer().print(&" ");
        conz::printer().println_type(&dt.str_dayname(), conz::MsgType::Value);
    }

    pub fn mk_point(state: &mut state::State, _: astr::AstrVec){
        conz::printer().println_type(&"Add deadline: ", conz::MsgType::Normal);
        let mut fields = wizard::FieldVec::new();
        fields.add(wizard::InputType::Text, astr::from_str("title: "), false);
        fields.add(wizard::InputType::Text, astr::from_str("type: "), false);
        fields.add(wizard::InputType::DateTime, astr::from_str("time date: "), true);
        let res = fields.execute();
        if res.is_err() {return;}
        let mut res = res.unwrap();
        let point = res.extract_point();
        if point.is_err() {return;}
        state.points.add_item(point.unwrap());
        if !state.points.write() {return;}
        conz::printer().println_type(&"Success: Point saved.", conz::MsgType::Highlight);
    }

    pub fn rm_point(state: &mut state::State, _: astr::AstrVec){
        conz::printer().println_type(&"Remove deadline: ", conz::MsgType::Normal);
        let mut fields = wizard::FieldVec::new();
        fields.add(wizard::InputType::Text, astr::from_str("title: "), false);
        fields.add(wizard::InputType::Text, astr::from_str("type: "), false);
        fields.add(wizard::InputType::Text, astr::from_str("time date: "), true);
        loop{
            let res = fields.execute();
            if res.is_err() {return;}
            let mut res = res.unwrap();
            let ptitle = astr::Astr::unwrap_default(res.get_text());
            let ptype = astr::Astr::unwrap_default(res.get_text());
            let pdt = data::DT::unwrap_default(res.get_dt());
            let mut score = 0;
            let mut more_than_one = false;
            let mut vec = Vec::new();
            let points = state.points.get_items();
            for i in 0..points.len(){
                let current = &points[i];
                let mut curr_score = 0;
                if ptitle == current.title{
                    curr_score += 1;
                }
                if data::PointType::from_astr(&ptype) == current.ptype{
                    curr_score += 1;
                }
                if pdt == current.dt{
                    curr_score += 1;
                }
                if curr_score > score{
                    score = curr_score;
                    more_than_one = false;
                    vec.clear();
                    vec.push(i);
                }
                else if curr_score == score{
                    more_than_one = true;
                    vec.push(i);
                }
            }
            if score > 0 && !more_than_one{
                conz::printer().println_type(&"Success: found a match:", conz::MsgType::Highlight);
                points[vec[0]].print();
                match conz::read_bool(&"Delete this item?: "){
                    true =>{
                        let ok = state.points.remove_indices(vec);
                        if ok {
                            conz::printer().println_type(&"Success: Item removed.", conz::MsgType::Highlight);
                        }else{
                            conz::printer().println_type(&"Error: Item removing failed.", conz::MsgType::Highlight);
                        }
                        return;
                    }
                    false =>{return;}
                }
            }
            if score == 0{
                conz::printer().println_type(&"Fail: no matches found.", conz::MsgType::Error);
                match conz::read_bool(&"Try again?: "){
                    true =>{continue;}
                    false =>{return;}
                }
            }
            if more_than_one{
                conz::printer().println_type(&"Fail: query is ambiguous.", conz::MsgType::Error);
                conz::printer().print_type(&"Found ", conz::MsgType::Normal);
                conz::printer().print_type(&format!("{}", vec.len()), conz::MsgType::Value);
                conz::printer().println_type(&" items.", conz::MsgType::Normal);
                for i in &vec{
                    points[*i].print();
                }
                match conz::read_bool(&"Delete all?: "){
                    true =>{
                        let ok = state.points.remove_indices(vec);
                        if ok {
                            conz::printer().println_type(&"Success: Items removed.", conz::MsgType::Highlight);
                        }else{
                            conz::printer().println_type(&"Error: Items removing failed.", conz::MsgType::Highlight);
                        }
                        return;
                    }
                    false =>{}
                }
                match conz::read_bool(&"Try again?: "){
                    true =>{continue;}
                    false =>{return;}
                }
            }
        }
    }

    pub fn ls_points(state: &mut state::State, _: astr::AstrVec){
        let count = state.points.get_items().len();
        let len_title = 32; let len_relative = 14; let len_dt = 23; let len_type = 11;
        conz::printer().print_type(&"Found ", conz::MsgType::Normal);
        conz::printer().print_type(&format!("{}", count), conz::MsgType::Value);
        conz::printer().println_type(&" points.", conz::MsgType::Normal);
        let divider_ver = || {conz::printer().print_type(&" | ", conz::MsgType::Highlight);};
        let divider_ver_edge = || {conz::printer().print_type(&"|", conz::MsgType::Highlight);};
        let divider_hor = |a| {astr::from_str("|")
            .concat(astr::from_str(a).repeat(len_title + len_relative + len_dt + len_type + (3*3)))
            .concat(astr::from_str("|"))};
        conz::printer().println_type(&divider_hor("="), conz::MsgType::Highlight);
        divider_ver_edge();
        conz::printer().print_type(
            &astr::from_str("title:").pad_after(len_title), 
            conz::MsgType::Normal);
        divider_ver();
        conz::printer().print_type(
            &astr::from_str("relative:").pad_after(len_relative), 
            conz::MsgType::Normal);
        divider_ver();
        conz::printer().print_type(
            &astr::from_str("time date:").pad_after(len_dt),
            conz::MsgType::Normal);
            divider_ver();
        conz::printer().print_type(
            &astr::from_str("type:").pad_after(len_type),
            conz::MsgType::Normal);
        divider_ver_edge();
        conz::printer().println(&"");
        conz::printer().println_type(&divider_hor("-"), conz::MsgType::Highlight);
        let now = data::DT::new();
        for x in state.points.get_items(){
            let diff = now.diff(&x.dt);
            let timecol = if diff.neg{
                conz::MsgType::Error
            }else if diff.total_hours <= 48 {
                conz::MsgType::Highlight
            }else{
                conz::MsgType::Normal
            };
            divider_ver_edge();
            conz::printer().print_type(
                &x.title.pad_after(len_title),
                conz::MsgType::Normal);
            divider_ver();
            conz::printer().print_type(
                &diff.string_significant()
                    .to_astr()
                    .pad_after(len_relative),
                timecol);
            divider_ver();
            conz::printer().print_type(
                &x.dt.str_datetime().concat(astr::from_str(" "))
                    .concat(x.dt.str_dayname_short()).pad_after(len_dt),
                conz::MsgType::Value);
            divider_ver();
            conz::printer().print_type(
                &x.ptype.to_astr().pad_after(len_type),
                conz::MsgType::Normal);
            divider_ver_edge();
            conz::printer().println(&"");
        }
        conz::printer().println_type(&divider_hor("="), conz::MsgType::Highlight);
    }

    pub fn flush_files(state: &mut state::State, _: astr::AstrVec){
        if state.is_clean() {
            conz::printer().println_type(&"All files clean, nothing to do.", conz::MsgType::Highlight);
            return;
        }
        let res = state.flush_files();
        if res {
            conz::printer().println_type(&"Success: Flushed all dirty files.", conz::MsgType::Highlight);
        }else{
            conz::printer().println_type(&"Error: Could not flush all dirty files.", conz::MsgType::Error);
        }
    }
}
