use std::io::prelude::*;
use std::fs::File;

use super::conz;
use super::conz::PrinterFunctions;
use super::conz::Printable;
use super::data;
use super::astr;
use super::astr::{AStr};
use super::state;
use super::misc::{UnwrapDefault};
use super::support;
use super::wizard;
use super::wizard::{Wizardable};

pub fn now(_: &mut state::State, _: astr::AstrVec){
    let dt = data::DT::new();
    pprint_type!(&dt.str_datetime(), conz::MsgType::Value);
    pprint!(&" ");
    pprintln_type!(&dt.str_dayname(), conz::MsgType::Value);
}

pub fn help(state: &mut state::State, args: astr::AstrVec){
    if args.len() == 0{
        pprint_type!(&"Help, type ", conz::MsgType::Normal);
        pprint_type!(&"help(command) ", conz::MsgType::Highlight);
        pprintln_type!(&"to find help.", conz::MsgType::Normal);
        pprint_type!(&"For example: ", conz::MsgType::Normal);
        pprint_type!(&"help (mk point)", conz::MsgType::Highlight);
        pprintln_type!(&".", conz::MsgType::Normal);
        pprint_type!(&"To list all commands use ", conz::MsgType::Normal);
        pprint_type!(&"ls commands", conz::MsgType::Highlight);
        pprintln_type!(&".", conz::MsgType::Normal);
        return;
    }
    let mut path = std::path::PathBuf::from("./help");
    let metatdata = std::fs::metadata(path.as_path());
    if metatdata.is_err(){
        pprintln_type!(&"Error: Help directory not found.", conz::MsgType::Error);
        return;
    }
    let res = state.fset.contains(&args[0]);
    if !res {
        pprintln_type!(&"Fail: command does not exist, so help for it neither.", conz::MsgType::Error);
        return;
    }
    path.push(astr::unsplit(&args[0].split_str(&astr::astr_whitespace()), '_' as u8).to_string());
    let res = std::fs::metadata(path.clone());
    if res.is_err(){
        pprintln_type!(&"Error: help file not found.", conz::MsgType::Error);
        return;
    }
    let f = File::open(path.as_path());
    if f.is_err(){
        pprintln_type!(&"Error: could not open file.", conz::MsgType::Error);
        return;
    }
    let mut f = f.unwrap();
    let mut string = String::new();
    let ok = f.read_to_string(&mut string);
    if ok.is_err(){
        pprintln_type!(&"Error: could not read file.", conz::MsgType::Error);
        return;
    }
    pprint_type!(&"Command: ", conz::MsgType::Normal);
    pprintln_type!(&astr::unsplit(&args, ' ' as u8).to_string(), conz::MsgType::Highlight);
    pprintln_type!(&string, conz::MsgType::Normal);
}

pub fn ls_commands(state: &mut state::State, _: astr::AstrVec){
    pprintln_type!(&"All commands: ", conz::MsgType::Normal);
    for f in &state.fset{
        pprintln_type!(f, conz::MsgType::Normal);
    }
}

pub fn mk_point(state: &mut state::State, _: astr::AstrVec){
    pprintln_type!(&"Add point: ", conz::MsgType::Normal);
    let fields = data::Point::get_fields(false);
    let res = fields.execute();
    if res.is_none() {return;}
    let mut res = res.unwrap();
    let point = data::Point::extract(&mut res);
    if point.is_none() {return;}
    state.points.add_item(point.unwrap());
    if !state.points.write() {return;}
    pprintln_type!(&"Success: Point saved.", conz::MsgType::Highlight);
}

pub fn rm_point(state: &mut state::State, _: astr::AstrVec){
    let items = state.points.get_items().clone();
    support::rm_items(items, &mut state.points, &mut state.points_archive);
}

pub fn clean_points(state: &mut state::State, _: astr::AstrVec){
    pprintln_type!(&"Remove all points that are in the past: ", conz::MsgType::Normal);
    match conz::read_bool(&"Sure to remove them?: "){
        true =>{}
        false =>{return;}
    }
    let points = state.points.get_items().clone();
    let mut vec = Vec::new();
    let now = data::DT::new();
    for i in 0..points.len(){
        if !now.diff(&points[i].dt).neg{
            break;
        }
        vec.push(i);
    }
    support::remove_and_archive(&mut state.points, &mut state.points_archive, vec, &points);
}

pub fn edit_point(state: &mut state::State, _: astr::AstrVec){
    pprintln_type!(&"Edit point(search first): ", conz::MsgType::Normal);
    let fields = data::Point::get_fields(true);
    let points = state.points.get_items();
    loop{
        let (match_res, vec) = support::get_matches(points);
        match match_res{
            support::MatchResult::None =>{
                pprintln_type!(&"Fail: no matches found.", conz::MsgType::Error);
                match conz::read_bool(&"Try again?: "){
                    true =>{continue;}
                    false =>{return;}
                }
            }
            support::MatchResult::Some =>{
                pprint_type!(&"Found ", conz::MsgType::Normal);
                pprint_type!(&format!("{}", vec.len()), conz::MsgType::Value);
                pprintln_type!(&" items.", conz::MsgType::Normal);
                for i in &vec{
                    points[*i].print();
                }
                match conz::read_bool(&"Edit all?: "){
                    true =>{
                        let mut replacements = Vec::new();
                        let mut indices = Vec::new();
                        for i in &vec{
                            let mut npoint = points[*i].clone();
                            npoint.print();
                            let res = fields.execute();
                            if res.is_none() {return;}
                            let mut res = res.unwrap();
                            let nptitle = astr::Astr::unwrap_default(res.get_text());
                            let nptype = data::PointType::from_astr(&astr::Astr::unwrap_default(res.get_text()), true);
                            let npdt = data::DT::unwrap_default(res.get_dt());
                            npoint.title.replace_if_not_default(nptitle);
                            npoint.ptype.replace_if_not_default(nptype);
                            npoint.dt.replace_if_not_default(npdt);
                            pprintln_type!(&"New item: ", conz::MsgType::Normal);
                            npoint.print();
                            let ok = conz::read_bool("Apply edit?: ");
                            if !ok {continue;}
                            indices.push(*i);
                            replacements.push(npoint);
                        }
                        let ok = state.points.replace(indices, replacements);
                        if ok {
                            pprintln_type!(&"Success: Items edited.", conz::MsgType::Highlight);
                        }else{
                            pprintln_type!(&"Error: Items editing failed.", conz::MsgType::Highlight);
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
}

pub fn ls_points(state: &mut state::State, _: astr::AstrVec){
    support::pretty_print(state.points.get_items(), data::DT::new());
}

pub fn ls_points_archive(state: &mut state::State, _: astr::AstrVec){
    let res = state.points_archive.read();
    support::pretty_print(&res, data::DT::new());
}

pub fn inspect_point(state: &mut state::State, _: astr::AstrVec){
    pprintln_type!(&"Inspect point(search first): ", conz::MsgType::Normal);
    loop{
        let points = state.points.get_items();
        let (match_res, vec) = support::get_matches(&points);
        if match_res == support::MatchResult::None || vec.len() > 1{
            if vec.len() > 1{
                pprintln_type!(&"Fail: more than one result.", conz::MsgType::Error);
            }else{
                pprintln_type!(&"Fail: no results found.", conz::MsgType::Error);
            }
            match conz::read_bool(&"Try again?: "){
                true =>{continue;}
                false =>{return;}
            }
        }
        points[vec[0]].print();
        let now = data::DT::new();
        let diff = now.diff(&points[vec[0]].dt);
        diff.print();
        return;
    }
}

pub fn mk_todo(state: &mut state::State, _: astr::AstrVec){
    pprintln_type!(&"Add todo: ", conz::MsgType::Normal);
    let fields = data::Todo::get_fields(false);
    let res = fields.execute();
    if res.is_none() {return;}
    let mut res = res.unwrap();
    let todo = data::Todo::extract(&mut res);
    if todo.is_none() {return;}
    state.todos.add_item(todo.unwrap());
    if !state.todos.write() {return;}
    pprintln_type!(&"Success: Todo saved.", conz::MsgType::Highlight);
}

pub fn ls_todos(state: &mut state::State, _: astr::AstrVec){
    let (to,lo,id) = support::split_todos(state.todos.get_items());
    pprintln_type!(&"Todo:", conz::MsgType::Normal);
    support::pretty_print(&to, 0);
    pprintln_type!(&"Longterm:", conz::MsgType::Normal);
    support::pretty_print(&lo, 0);
    pprintln_type!(&"Idea:", conz::MsgType::Normal);
    support::pretty_print(&id, 0);
}

pub fn rm_todos(state: &mut state::State, _: astr::AstrVec){

}

pub fn flush_files(state: &mut state::State, _: astr::AstrVec){
    if state.is_clean() {
        pprintln_type!(&"All files clean, nothing to do.", conz::MsgType::Highlight);
        return;
    }
    let res = state.flush_files();
    if res {
        pprintln_type!(&"Success: Flushed all dirty files.", conz::MsgType::Highlight);
    }else{
        pprintln_type!(&"Error: Could not flush all dirty files.", conz::MsgType::Error);
    }
}
