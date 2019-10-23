use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;
use term_basics_linux as tbl;

use super::conz;
use super::conz::Printable;
use super::data;
use super::astr;
use super::astr::{AStr};
use super::state;
use super::support;
use super::save;

pub fn missing_help(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_inputs(&inputs);
    support::warn_unused_arguments(&args);
    let mut missing = Vec::new();
    for f in state.fset.clone(){
        let mut path = std::path::PathBuf::from("./help");
        let mut metatdata = std::fs::metadata(path.as_path());
        if metatdata.is_err(){
            let res = save::get_data_dir_path("help");
            if res.is_some(){
                path = res.unwrap();
                metatdata = std::fs::metadata(path.as_path());
            }
        }
        if metatdata.is_err(){
            conz::println_type("Error: Help directory not found.", conz::MsgType::Error);
            return;
        }
        let res = state.fset.contains(&f);
        if !res { // really should not happen xd
            conz::println_type("Fail: command does not exist, so help for it neither.", conz::MsgType::Error);
            continue;
        }
        path.push(astr::unsplit(&f.split_str(&astr::astr_whitespace()), '_' as u8).to_string());
        let res = std::fs::metadata(path.clone());
        if res.is_err(){
            missing.push(f);
        }
    }
    conz::println_type("These commands do exist but have no help file: ", conz::MsgType::Highlight);
    missing.sort();
    for c in missing{
        conz::println_type(c.disp(), conz::MsgType::Normal);
    }
}

pub fn help_cli(){
    conz::println_type("pplanner is an TUI/CLI program to manage your time.", conz::MsgType::Normal);
    conz::println_type("To use it, start it and type commands in its prompt.", conz::MsgType::Normal);
    conz::print_type("Type ", conz::MsgType::Normal);
    conz::print_type("help", conz::MsgType::Highlight);
    conz::println_type(" in its prompt to get help on commands.", conz::MsgType::Normal);
    conz::println_type("Give a pplanner command as cli argument to run it directly from the terminal.", conz::MsgType::Normal);
    conz::print_type("For example: ", conz::MsgType::Normal);
    conz::println_type("pplanner \'ls todos\'", conz::MsgType::Highlight);
    conz::print_type("pplanner is made by ", conz::MsgType::Normal);
    conz::println_type("Cody Bloemhard.", conz::MsgType::Prompt);
}

pub fn now(_: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let dt = data::DT::new();
    conz::print_type("Time:  ", conz::MsgType::Normal);
    conz::println_type(dt.str_time().disp(), conz::MsgType::Value);
    conz::print_type("Date:  ", conz::MsgType::Normal);
    conz::println_type(dt.str_date().disp(), conz::MsgType::Value);
    conz::print_type("Week:  ", conz::MsgType::Normal);
    conz::println_type(dt.str_weeknr().disp(), conz::MsgType::Value);
    conz::print_type("Day:   ", conz::MsgType::Normal);
    conz::println_type(dt.str_dayname().disp(), conz::MsgType::Value);
    conz::print_type("Month: ", conz::MsgType::Normal);
    conz::println_type(dt.str_monthname().disp(), conz::MsgType::Value);
}

pub fn license(_: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let path = save::get_data_dir_path("LICENSE");
    if path.is_none(){
        conz::println_type("Error: Could not find license file.", conz::MsgType::Error);
        return;
    }
    let path = path.unwrap();
    let metatdata = std::fs::metadata(path.as_path());
    if metatdata.is_err(){
        conz::println_type("Error: Could not find license file.", conz::MsgType::Error);
        return;
    }
    let f = File::open(path.as_path());
    if f.is_err(){
        conz::println_type("Error: could not open file.", conz::MsgType::Error);
        return;
    }
    let mut f = f.unwrap();
    let mut string = String::new();
    let ok = f.read_to_string(&mut string);
    if ok.is_err(){
        conz::println_type("Error: could not read file.", conz::MsgType::Error);
        return;
    }
    conz::println_type(string, conz::MsgType::Normal);
}

pub fn help(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_inputs(&inputs);
    if args.len() == 0{
        conz::print_type("Help, type ", conz::MsgType::Normal);
        conz::print_type("help(command) ", conz::MsgType::Highlight);
        conz::println_type("to find help.", conz::MsgType::Normal);
        conz::print_type("For example: ", conz::MsgType::Normal);
        conz::print_type("help (mk point)", conz::MsgType::Highlight);
        conz::println_type(".", conz::MsgType::Normal);
        conz::print_type("To list all commands use ", conz::MsgType::Normal);
        conz::print_type("ls commands", conz::MsgType::Highlight);
        conz::println_type(".", conz::MsgType::Normal);
        return;
    }
    let mut path = std::path::PathBuf::from("./help");
    let mut metatdata = std::fs::metadata(path.as_path());
    if metatdata.is_err(){
        let res = save::get_data_dir_path("help");
        if res.is_some(){
            path = res.unwrap();
            metatdata = std::fs::metadata(path.as_path());
        }
    }
    if metatdata.is_err(){
        conz::println_type("Error: Help directory not found.", conz::MsgType::Error);
        return;
    }
    let res = state.fset.contains(&args[0]);
    if !res {
        conz::println_type("Fail: command does not exist, so help for it neither.", conz::MsgType::Error);
        return;
    }
    path.push(astr::unsplit(&args[0].split_str(&astr::astr_whitespace()), '_' as u8).to_string());
    let res = std::fs::metadata(path.clone());
    if res.is_err(){
        conz::println_type("Error: help file not found.", conz::MsgType::Error);
        return;
    }
    let f = File::open(path.as_path());
    if f.is_err(){
        conz::println_type("Error: could not open file.", conz::MsgType::Error);
        return;
    }
    let mut f = f.unwrap();
    let mut string = String::new();
    let ok = f.read_to_string(&mut string);
    if ok.is_err(){
        conz::println_type("Error: could not read file.", conz::MsgType::Error);
        return;
    }
    conz::print_type("Command: ", conz::MsgType::Normal);
    conz::println_type(astr::unsplit(&args, ' ' as u8).to_string(), conz::MsgType::Highlight);
    conz::println_type(string, conz::MsgType::Normal);
}

pub fn ls_days(_: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    conz::println_type("All names of the days in the week: ", conz::MsgType::Normal);
    for i in 0..7 {
        conz::print_type(format!("{} ", i), conz::MsgType::Normal);
        conz::print_type(data::day_name(i).disp(), conz::MsgType::Value);
        conz::print_type(" (", conz::MsgType::Normal);
        conz::print_type(format!("{}", data::day_name_short(i).disp()), conz::MsgType::Value);
        conz::println_type(")", conz::MsgType::Normal);
    }
}

pub fn ls_months(_: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    conz::println_type("All names of the months in the year: ", conz::MsgType::Normal);
    for i in 1..13 {
        conz::print_type(format!("{} ", i), conz::MsgType::Normal);
        if i < 10 { conz::print_type(" ", conz::MsgType::Normal); }
        conz::print_type(data::month_name(i).disp(), conz::MsgType::Value);
        conz::print_type(" (", conz::MsgType::Normal);
        conz::print_type(data::month_name_short(i).disp(), conz::MsgType::Value);
        conz::println_type(")", conz::MsgType::Normal);
    }
}

pub fn ls_commands(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    conz::println_type("All commands: ", conz::MsgType::Normal);
    let mut commands = Vec::new();
    for f in state.fset.clone(){
        commands.push(f);
    }
    commands.sort();
    for f in commands{
        conz::println_type(f.disp(), conz::MsgType::Normal);
    }
}

pub fn mk_point(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::mk_item(&mut state.points, &mut inputs);
}

pub fn rm_points(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    let items = state.points.get_items().clone();
    support::rm_items(items, &mut state.points, &mut state.points_archive, &mut inputs);
}

pub fn clean_points(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    conz::println_type("Remove all points that are in the past: ", conz::MsgType::Normal);
    match conz::read_bool("Sure to remove them?: ", &mut inputs){
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

pub fn edit_points(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    check_unsupported_inputs!(inputs);
    support::edit_items(&mut state.points);
}

pub fn ls_points(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    support::pretty_print(state.points.get_items(), &data::DT::new());
}

pub fn ls_points_archive(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let res = state.points_archive.read();
    support::pretty_print(&res, &data::DT::new());
}

pub fn inspect_point(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    conz::println_type("Inspect point(search first): ", conz::MsgType::Normal);
    loop{
        let points = state.points.get_items();
        let (match_res, vec) = support::get_matches(&points,&mut inputs);
        if match_res == support::MatchResult::None || vec.len() > 1{
            if vec.len() > 1{
                conz::println_type("Fail: more than one result.", conz::MsgType::Error);
            }else{
                conz::println_type("Fail: no results found.", conz::MsgType::Error);
            }
            if inputs.is_some() {return;}
            match conz::read_bool("Try again?: ", &mut Option::None){
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

pub fn mk_todo(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::mk_item(&mut state.todos, &mut inputs);
}

pub fn rm_todos(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    let items = state.todos.get_items().clone();
    support::rm_items(items, &mut state.todos, &mut state.todos_archive, &mut inputs);
}

pub fn edit_todos(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    check_unsupported_inputs!(inputs);
    support::edit_items(&mut state.todos);
}

pub fn ls_todos(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let (doi,tod,lon,ide) = support::split_todos(state.todos.get_items());
    conz::print_type("Doing: ", conz::MsgType::Normal);
    support::pretty_print(&doi, &false);
    conz::print_type("Todo: ", conz::MsgType::Normal);
    support::pretty_print(&tod, &false);
    conz::print_type("Longterm: ", conz::MsgType::Normal);
    support::pretty_print(&lon, &false);
    conz::print_type("Idea: ", conz::MsgType::Normal);
    support::pretty_print(&ide, &false);
}

pub fn ls_todos_archive(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let res = state.todos_archive.read();
    support::pretty_print(&res, &true);
}

pub fn status(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    now(state, args.clone(), inputs.clone());
    ls_points(state, args.clone(), inputs.clone());
    ls_todos(state, args.clone(), inputs.clone());
}

pub fn mk_slice(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::mk_item(&mut state.slices, &mut inputs);
}

pub fn rm_slices(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    let items = state.slices.get_items().clone();
    support::rm_items(items, &mut state.slices, &mut state.slices_archive, &mut inputs);
}

pub fn clean_slices(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    conz::println_type("Remove all slices that are in the past: ", conz::MsgType::Normal);
    match conz::read_bool("Sure to remove them?: ", &mut inputs){
        true =>{}
        false =>{return;}
    }
    let slices = state.slices.get_items().clone();
    let mut vec = Vec::new();
    let now = data::DT::new();
    for i in 0..slices.len(){
        if !now.diff(&slices[i].start).neg{
            break;
        }
        vec.push(i);
    }
    support::remove_and_archive(&mut state.slices, &mut state.slices_archive, vec, &slices);
}

pub fn edit_slices(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    check_unsupported_inputs!(inputs);
    support::edit_items(&mut state.slices);
}

pub fn ls_slices(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    support::pretty_print(state.slices.get_items(), &0);
}

pub fn ls_slices_archive(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    let res = state.slices_archive.read();
    support::pretty_print(&res, &0);
}

pub fn inspect_slice(state: &mut state::State, args: astr::AstrVec, mut inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    conz::println_type("Inspect slice(search first): ", conz::MsgType::Normal);
    loop{
        let slices = state.slices.get_items();
        let (match_res, vec) = support::get_matches(&slices,&mut inputs);
        if match_res == support::MatchResult::None || vec.len() > 1{
            if vec.len() > 1{
                conz::println_type("Fail: more than one result.", conz::MsgType::Error);
            }else{
                conz::println_type("Fail: no results found.", conz::MsgType::Error);
            }
            if inputs.is_some() {return;}
            match conz::read_bool("Try again?: ", &mut Option::None){
                true =>{continue;}
                false =>{return;}
            }
        }
        let slice = &slices[vec[0]]; 
        slice.print();
        let now = data::DT::new();
        conz::println_type("Duration: ", conz::MsgType::Highlight);
        let diff = slice.end.diff(&slice.start);
        diff.print_as_duration();
        conz::println_type("Start:", conz::MsgType::Highlight);
        let diff = now.diff(&slice.start);
        diff.print();
        conz::println_type("End:", conz::MsgType::Highlight);
        let diff = now.diff(&slice.end);
        diff.print();
        return;
    }
}

pub fn flush_files(state: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    if state.is_clean() {
        conz::println_type("All files clean, nothing to do.", conz::MsgType::Highlight);
        return;
    }
    let res = state.flush_files();
    if res {
        conz::println_type("Success: Flushed all dirty files.", conz::MsgType::Highlight);
    }else{
        conz::println_type("Error: Could not flush all dirty files.", conz::MsgType::Error);
    }
}

pub fn test_keys(_: &mut state::State, args: astr::AstrVec, inputs: Option<VecDeque<astr::Astr>>){
    support::warn_unused_arguments(&args);
    support::warn_unused_inputs(&inputs);
    conz::println_type("Testing keys, press any key to get id, exit program to stop.", conz::MsgType::Normal);
    tbl::test_chars();
}
