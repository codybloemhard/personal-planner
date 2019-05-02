use super::astr;
use super::astr::{AStr};
use super::data;
use super::misc::{UnwrapDefault};
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
    let res = fields.execute();
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

pub fn remove_and_archive(bf: &mut save::BufferFile<data::Point>, af: &mut save::ArchiveFile<data::Point>, 
    vec: Vec<usize>, points: &Vec<data::Point>){
    let ok = bf.remove_indices(vec.clone());
    if ok {
        pprintln_type!(&"Success: Items removed.", conz::MsgType::Highlight);
    }else{
        pprintln_type!(&"Error: Items removing failed.", conz::MsgType::Highlight);
        return;
    }
    for i in &vec{
        af.add_item(points[*i].clone());
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

pub fn pretty_print<T: conz::PrettyPrintable>(datavec: &Vec<T>, arg: T::ArgType){
    let count = datavec.len();
    let lengths = T::lengths();
    let titles = T::titles();
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
        let (texts,types) = x.pretty_print(&arg);
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
