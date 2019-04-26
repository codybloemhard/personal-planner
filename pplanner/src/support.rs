use super::wizard;
use super::astr;
use super::data;
use super::misc::{UnwrapDefault};
use super::save;
use super::conz;
use super::conz::{PrinterFunctions};

pub fn get_point_fields(partial: bool) -> wizard::FieldVec{
    let mut fields = wizard::FieldVec::new();
    if partial{
        fields.add(wizard::InputType::Text, astr::from_str("title: "), wizard::PromptType::Partial);
        fields.add(wizard::InputType::Text, astr::from_str("type: "), wizard::PromptType::Partial);
        fields.add(wizard::InputType::DateTime, astr::from_str("time date: "), wizard::PromptType::Partial);
    }else{
        fields.add(wizard::InputType::Text, astr::from_str("title: "), wizard::PromptType::Once);
        fields.add(wizard::InputType::Text, astr::from_str("type: "), wizard::PromptType::Once);
        fields.add(wizard::InputType::DateTime, astr::from_str("time date: "), wizard::PromptType::Reprompt);
    }
    return fields;
}

pub enum MatchResult{
    None,
    Some,
}

pub fn get_matches(points: &Vec<data::Point>) -> (MatchResult,Vec<usize>){
    let fields = get_point_fields(true);
    let res = fields.execute();
    if res.is_none() {
        return (MatchResult::None, Vec::new());
    }
    let mut res = res.unwrap();
    let ptitle = astr::Astr::unwrap_default(res.get_text());
    let ptype = data::PointType::from_astr(&astr::Astr::unwrap_default(res.get_text()));
    let pdt = data::DT::unwrap_default(res.get_dt());
    let mut score = 0;
    let mut vec = Vec::new();
    for i in 0..points.len(){
        let current = &points[i];
        let mut curr_score = 0;
        if ptitle == current.title{
            curr_score += 1;
        }
        if ptype == current.ptype{
            curr_score += 1;
        }
        if pdt == current.dt{
            curr_score += 1;
        }
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
    vec: Vec<usize>, points: Vec<data::Point>){
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