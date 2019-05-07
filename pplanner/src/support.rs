use super::astr;
use super::astr::{AStr};
use super::data;
use super::save;
use super::conz;
use super::conz::{PrinterFunctions};
use super::wizard::{Wizardable};

#[derive(PartialEq)]
pub enum MatchResult{
    None,
    Some,
}

pub fn get_matches<T: Wizardable>(data: &Vec<T>) -> (MatchResult,Vec<usize>){
    let fields = T::get_fields(true);
    let res = fields.execute(Vec::new());
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
        pprintln_type!(&"Success: Items removed.", conz::MsgType::Highlight);
    }else{
        pprintln_type!(&"Error: Items removing failed.", conz::MsgType::Highlight);
        return;
    }
    for i in &vec{
        af.add_item(data[*i].clone());
    }
    if !af.write(){
        pprintln_type!(&"Error: Could not write items to archive.", conz::MsgType::Error);
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
    pprint_type!(&"Found ", conz::MsgType::Normal);
    pprint_type!(&format!("{}", count), conz::MsgType::Value);
    pprintln_type!(&" items.", conz::MsgType::Normal);
    let divider_ver = || {pprint_type!(&" | ", conz::MsgType::Highlight);};
    let divider_ver_edge = || {pprint_type!(&"|", conz::MsgType::Highlight);};
    let divider_hor = |a| {astr::from_str("|")
        .concat(astr::from_str(a).repeat(lensum + ((lengths.len()-1)*3) as u16))
        .concat(astr::from_str("|"))};
    pprintln_type!(&divider_hor("="), conz::MsgType::Highlight);
    divider_ver_edge();
    for i in 0..titles.len() - 1{
        pprint_type!(
            &titles[i].pad_after(lengths[i]), 
            conz::MsgType::Normal);
        divider_ver();
    }
    pprint_type!(
        &titles[titles.len() - 1].pad_after(lengths[titles.len() - 1]), 
        conz::MsgType::Normal);
    divider_ver_edge();
    pprintln!(&"");
    pprintln_type!(&divider_hor("-"), conz::MsgType::Highlight);
    for x in datavec{
        divider_ver_edge();
        let (texts,types) = x.pretty_print(arg);
        if texts.len() != types.len(){
            panic!("Panic: pretty_print: texts.len() != types.len().");
        }
        for i in 0..texts.len() - 1{
            pprint_type!(
                &texts[i].pad_after(lengths[i]),
                types[i].clone());
            divider_ver();
        }
        pprint_type!(
            &texts[texts.len() - 1].pad_after(lengths[texts.len() - 1]),
            types[texts.len() - 1].clone());
        divider_ver_edge();
        pprintln!(&"");
    }
    pprintln_type!(&divider_hor("="), conz::MsgType::Highlight);
}

pub fn rm_items<T: Wizardable + save::Bufferable + std::cmp::Ord + Clone>
    (items: Vec<T>, bf: &mut save::BufferFile<T>, af: &mut save::ArchiveFile<T>){
    pprintln_type!(&"Remove point(search first): ", conz::MsgType::Normal);
    loop{
        let (match_res, vec) = get_matches(&items);
        match match_res{
            MatchResult::None =>{
                pprintln_type!(&"Fail: no matches found.", conz::MsgType::Error);
                match conz::read_bool(&"Try again?: "){
                    true =>{continue;}
                    false =>{return;}
                }
            }
            MatchResult::Some =>{
                pprint_type!(&"Found ", conz::MsgType::Normal);
                pprint_type!(&format!("{}", vec.len()), conz::MsgType::Value);
                pprintln_type!(&" items.", conz::MsgType::Normal);
                for i in &vec{
                    items[*i].print();
                }
                match conz::read_bool(&"Delete all?: "){
                    true =>{}
                    false =>{
                        match conz::read_bool(&"Try again?: "){
                            true =>{continue;}
                            false =>{return;}
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
    pprintln_type!(&"Edit point(search first): ", conz::MsgType::Normal);
    let fields = T::get_fields(true);
    let items = bf.get_items();
    loop{
        let (match_res, vec) = get_matches(items);
        match match_res{
            MatchResult::None =>{
                pprintln_type!(&"Fail: no matches found.", conz::MsgType::Error);
                match conz::read_bool(&"Try again?: "){
                    true =>{continue;}
                    false =>{return;}
                }
            }
            MatchResult::Some =>{
                pprint_type!(&"Found ", conz::MsgType::Normal);
                pprint_type!(&format!("{}", vec.len()), conz::MsgType::Value);
                pprintln_type!(&" items.", conz::MsgType::Normal);
                for i in &vec{
                    items[*i].print();
                }
                match conz::read_bool(&"Edit all?: "){
                    true =>{
                        let mut replacements = Vec::new();
                        let mut indices = Vec::new();
                        for i in &vec{
                            let mut npoint = items[*i].clone();
                            npoint.print();
                            let res = fields.execute(Vec::new());
                            if res.is_none() {return;}
                            let mut res = res.unwrap();
                            let partial = T::get_partial(&mut res);
                            npoint.replace_parts(&partial);
                            pprintln_type!(&"New item: ", conz::MsgType::Normal);
                            npoint.print();
                            let ok = conz::read_bool("Apply edit?: ");
                            if !ok {continue;}
                            indices.push(*i);
                            replacements.push(npoint);
                        }
                        let ok = bf.replace(indices, replacements);
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

pub fn split_todos(todos: &Vec<data::Todo>) -> (Vec<data::Todo>,Vec<data::Todo>,Vec<data::Todo>){
    let mut to = Vec::new();
    let mut lo = Vec::new();
    let mut id = Vec::new();
    let mut index = 0;
    for i in 0..todos.len(){
        if todos[i].ttype == data::TodoType::Long{
            index = i;
            break;
        }
        to.push(todos[i].clone());
    }
    for i in index..todos.len(){
        if todos[i].ttype == data::TodoType::Idea{
            index = i;
            break;
        }
        lo.push(todos[i].clone());
    }
    for i in index..todos.len(){
        id.push(todos[i].clone());
    }
    return (to,lo,id);
}

pub fn warn_unused_inputs(inputs: &Vec<astr::Astr>){
    if inputs.len() < 1 {return;}
    pprintln_type!(&"Warning: Inputs for this command where specified but this command does not use any.", conz::MsgType::Error);
}
