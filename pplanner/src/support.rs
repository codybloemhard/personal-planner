use std::collections::VecDeque;
use term_basics_linux as tbl;

use super::astr;
use super::astr::{AStr};
use super::data;
use super::save;
use super::conz;
use super::wizard::{Wizardable};

#[derive(PartialEq)]
pub enum MatchResult{
    None,
    Some,
}

pub fn get_matches<T: Wizardable>(data: &Vec<T>, inputs: &mut Option<VecDeque<astr::Astr>>) -> (MatchResult,Vec<usize>){
    let fields = T::get_fields(true);
    let res = fields.execute(inputs);
    if res.is_none(){
        return (MatchResult::None, Vec::new());
    }
    let mut res = res.unwrap();
    let partial = T::get_partial(&mut res);
    let mut score = 0;
    let mut vec = Vec::new();
    for i in 0..data.len(){
        let current = &data[i];
        let curr_score = partial.score_againts(current);
        if curr_score > score{
            score = curr_score;
            vec.clear();
            vec.push(i);
        }
        else if curr_score == score{
            vec.push(i);
        }
    }
    if score == 0{
        return (MatchResult::None, vec);
    }
    if score > 0{
        return (MatchResult::Some, vec);
    }
    //should not be reachable
    return (MatchResult::None, vec);
}

pub fn remove_and_archive<T: save::Bufferable + std::cmp::Ord + Clone>
    (bf: &mut save::BufferFile<T>, af: &mut save::ArchiveFile<T>, 
    vec: Vec<usize>, data: &Vec<T>){
    let ok = bf.remove_indices(vec.clone());
    if ok {
        conz::println_type("Success: Items removed.", conz::MsgType::Highlight);
    }else{
        conz::println_type("Error: Items removing failed.", conz::MsgType::Highlight);
        return;
    }
    for i in &vec{
        af.add_item(data[*i].clone());
    }
    if !af.write(){
        conz::println_type("Error: Could not write items to archive.", conz::MsgType::Error);
    }
}

pub fn diff_color(diff: &data::Span) -> conz::MsgType{
    if diff.neg{
        conz::MsgType::Error
    }else if diff.total_hours <= 48 {
        conz::MsgType::Highlight
    }else{
        conz::MsgType::Normal
    }
}

pub fn pretty_print<T: conz::PrettyPrintable>(datavec: &Vec<T>, arg: &T::ArgType){
    let count = datavec.len();
    let lengths = T::lengths(arg);
    let titles = T::titles(arg);
    if lengths.len() != titles.len() {
        panic!("Panic: pretty_print: lengths.len() != titles.len().");
    }
    let mut lensum = 0;
    for len in &lengths{
        lensum += len;
    }
    conz::print_type("Found ", conz::MsgType::Normal);
    conz::print_type(format!("{}", count), conz::MsgType::Value);
    conz::println_type(" items.", conz::MsgType::Normal);
    let divider_ver = || {conz::print_type(" | ", conz::MsgType::Highlight);};
    let divider_ver_edge = || {conz::print_type("|", conz::MsgType::Highlight);};
    let divider_hor = |a| {astr::from_str("|")
        .concat(astr::from_str(a).repeat(lensum + ((lengths.len()-1)*3) as u16))
        .concat(astr::from_str("|"))};
    conz::println_type(divider_hor("=").disp(), conz::MsgType::Highlight);
    divider_ver_edge();
    for i in 0..titles.len() - 1{
        conz::print_type(
            titles[i].pad_after(lengths[i]).disp(), 
            conz::MsgType::Normal);
        divider_ver();
    }
    conz::print_type(
        titles[titles.len() - 1].pad_after(lengths[titles.len() - 1]).disp(), 
        conz::MsgType::Normal);
    divider_ver_edge();
    tbl::println("");
    conz::println_type(divider_hor("-").disp(), conz::MsgType::Highlight);
    for x in datavec{
        divider_ver_edge();
        let (texts,types) = x.pretty_print(arg);
        if texts.len() != types.len(){
            panic!("Panic: pretty_print: texts.len() != types.len().");
        }
        for i in 0..texts.len() - 1{
            conz::print_type(
                texts[i].pad_after(lengths[i]).disp(),
                types[i].clone());
            divider_ver();
        }
        conz::print_type(
            texts[texts.len() - 1].pad_after(lengths[texts.len() - 1]).disp(),
            types[texts.len() - 1].clone());
        divider_ver_edge();
        tbl::println("");
    }
    conz::println_type(divider_hor("=").disp(), conz::MsgType::Highlight);
}

pub fn rm_items<T: Wizardable + save::Bufferable + std::cmp::Ord + Clone>
    (items: Vec<T>, bf: &mut save::BufferFile<T>, af: &mut save::ArchiveFile<T>,
    inputs: &mut Option<VecDeque<astr::Astr>>){
    conz::print_type("Remove ", conz::MsgType::Normal);
    conz::print_type(T::get_name().disp(), conz::MsgType::Normal);
    conz::println_type("(search first): ", conz::MsgType::Normal);
    let cli = inputs.is_some();
    loop{
        let (match_res, vec) = get_matches(&items, inputs);
        match match_res{
            MatchResult::None =>{
                conz::println_type("Fail: no matches found.", conz::MsgType::Error);
                if cli {return;}
                match conz::read_bool("Try again?: ", inputs){
                    true =>{continue;}
                    false =>{return;}
                }
            }
            MatchResult::Some =>{
                conz::print_type("Found ", conz::MsgType::Normal);
                conz::print_type(format!("{}", vec.len()), conz::MsgType::Value);
                conz::println_type(" items.", conz::MsgType::Normal);
                for i in &vec{
                    items[*i].print();
                }
                if !cli{
                    match conz::read_bool("Delete all?: ", inputs){
                        true =>{}
                        false =>{
                            match conz::read_bool("Try again?: ", inputs){
                                true =>{continue;}
                                false =>{return;}
                            }
                        }
                    }
                }
                remove_and_archive(bf, af, vec, &items);
                return;
            }
        }
    }
}

pub fn edit_items<T: Wizardable + save::Bufferable + std::cmp::Ord + Clone>
    (bf: &mut save::BufferFile<T>){
    conz::print_type("Edit ", conz::MsgType::Normal);
    conz::print_type(T::get_name().disp(), conz::MsgType::Normal);
    conz::println_type("(search first): ", conz::MsgType::Normal);
    let fields = T::get_fields(true);
    let items = bf.get_items();
    loop{
        let (match_res, vec) = get_matches(items, &mut Option::None);
        match match_res{
            MatchResult::None =>{
                conz::println_type("Fail: no matches found.", conz::MsgType::Error);
                match conz::read_bool("Try again?: ", &mut Option::None){
                    true =>{continue;}
                    false =>{return;}
                }
            }
            MatchResult::Some =>{
                conz::print_type("Found ", conz::MsgType::Normal);
                conz::print_type(format!("{}", vec.len()), conz::MsgType::Value);
                conz::println_type(" items.", conz::MsgType::Normal);
                for i in &vec{
                    items[*i].print();
                }
                match conz::read_bool("Edit all?: ", &mut Option::None){
                    true =>{
                        let mut replacements = Vec::new();
                        let mut indices = Vec::new();
                        for i in &vec{
                            let mut npoint = items[*i].clone();
                            npoint.print();
                            let res = fields.execute(&mut Option::None);
                            if res.is_none() {return;}
                            let mut res = res.unwrap();
                            let partial = T::get_partial(&mut res);
                            npoint.replace_parts(&partial);
                            conz::println_type("New item: ", conz::MsgType::Normal);
                            npoint.print();
                            let ok = conz::read_bool("Apply edit?: ", &mut Option::None);
                            if !ok {continue;}
                            indices.push(*i);
                            replacements.push(npoint);
                        }
                        let ok = bf.replace(indices, replacements);
                        if ok {
                            conz::println_type("Success: Items edited.", conz::MsgType::Highlight);
                        }else{
                            conz::println_type("Error: Items editing failed.", conz::MsgType::Highlight);
                        }
                        return;
                    }
                    false =>{}
                }
                match conz::read_bool("Try again?: ", &mut Option::None){
                    true =>{continue;}
                    false =>{return;}
                }
            }
        }
    }
}

pub fn split_todos(todos: &Vec<data::Plan>) -> (Vec<data::Plan>,Vec<data::Plan>,Vec<data::Plan>,Vec<data::Plan>){
    let mut doi = Vec::new();
    let mut tod = Vec::new();
    let mut lon = Vec::new();
    let mut ide = Vec::new();
    let mut index = 0;
    loop{
        for i in 0..todos.len(){
            index = i;
            if todos[i].ttype == data::PlanType::Long { break; }
            tod.push(todos[i].clone());
        }
        for i in index..todos.len(){
            index = i;
            if todos[i].ttype == data::PlanType::Idea { break; }
            lon.push(todos[i].clone());
        }
        for i in index..todos.len(){
            index = i;
            if todos[i].ttype == data::PlanType::Current { break; }
            ide.push(todos[i].clone());
        }
        for i in index..todos.len(){
            doi.push(todos[i].clone());
        }
        break;
    }
    return (doi,tod,lon,ide);
}

pub fn mk_item<T: Wizardable + save::Bufferable + std::cmp::Ord + Clone>
    (bfile: &mut save::BufferFile<T>, inputs: &mut Option<VecDeque<astr::Astr>>){
    conz::print_type("Add ", conz::MsgType::Normal);
    conz::print_type(T::get_name().disp(), conz::MsgType::Normal);
    conz::println_type(": ", conz::MsgType::Normal);
    let fields = T::get_fields(false);
    let res = fields.execute(inputs);
    if res.is_none() {return;}
    let mut res = res.unwrap();
    let item = T::extract(&mut res);
    if item.is_none() {return;}
    bfile.add_item(item.unwrap());
    if !bfile.write() {return;}
    conz::print_type("Success: ", conz::MsgType::Highlight);
    conz::print_type(T::get_name().disp(), conz::MsgType::Highlight);
    conz::println_type(" saved!", conz::MsgType::Highlight);
}

pub fn warn_unused_inputs(inputs: &Option<VecDeque<astr::Astr>>){
    if inputs.is_none() {return;}
    conz::println_type("Warning: Inputs for this command where specified but this command does not use any.", conz::MsgType::Error);
}

pub fn warn_unused_arguments(args: &Vec<astr::Astr>){
    if args.len() < 1 {return;}
    conz::println_type("Warning: Arguments for this command where specified but this command does not use any.", conz::MsgType::Error);
}

#[macro_export]
macro_rules! check_unsupported_inputs{
    ($inputs:expr) => {
        if $inputs.is_some() {
            conz::println_type("Error: this command does not support execution with givin inputs fro the cli.",
                conz::MsgType::Error);
            return;
        }
    };
}
