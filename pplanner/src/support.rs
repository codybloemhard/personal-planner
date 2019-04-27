use super::wizard;
use super::astr;
use super::astr::{AStr,ToAstr};
use super::data;
use super::misc::{UnwrapDefault};
use super::save;
use super::conz;
use super::conz::{PrinterFunctions};

pub fn get_point_fields(partial: bool) -> wizard::FieldVec{
    let mut fields = wizard::FieldVec::new();
    if partial{
        fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Partial);
        fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Partial);
        fields.add(wizard::InputType::DateTime, astr::from_str("Time date: "), wizard::PromptType::Partial);
    }else{
        fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Once);
        fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Once);
        fields.add(wizard::InputType::DateTime, astr::from_str("Time date: "), wizard::PromptType::Reprompt);
    }
    return fields;
}

pub fn get_todo_fields(partial: bool) -> wizard::FieldVec{
    let mut fields = wizard::FieldVec::new();
    if partial{
        fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Partial);
        fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Partial);
        fields.add(wizard::InputType::U16, astr::from_str("Urgency: "), wizard::PromptType::Partial);
    }else{
        fields.add(wizard::InputType::Text, astr::from_str("Title: "), wizard::PromptType::Once);
        fields.add(wizard::InputType::Text, astr::from_str("Type: "), wizard::PromptType::Once);
        fields.add(wizard::InputType::U16, astr::from_str("Urgency: "), wizard::PromptType::Reprompt);
    }
    return fields;
}

#[derive(PartialEq)]
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
    let ptype = data::PointType::from_astr(&astr::Astr::unwrap_default(res.get_text()), true);
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

pub fn print_points(points: &Vec<data::Point>){
    let count = points.len();
    let len_title = 32; let len_relative = 14; let len_dt = 23; let len_type = 11;
    pprint_type!(&"Found ", conz::MsgType::Normal);
    pprint_type!(&format!("{}", count), conz::MsgType::Value);
    pprintln_type!(&" points.", conz::MsgType::Normal);
    let divider_ver = || {pprint_type!(&" | ", conz::MsgType::Highlight);};
    let divider_ver_edge = || {pprint_type!(&"|", conz::MsgType::Highlight);};
    let divider_hor = |a| {astr::from_str("|")
        .concat(astr::from_str(a).repeat(len_title + len_relative + len_dt + len_type + (3*3)))
        .concat(astr::from_str("|"))};
    pprintln_type!(&divider_hor("="), conz::MsgType::Highlight);
    divider_ver_edge();
    pprint_type!(
        &astr::from_str("title:").pad_after(len_title), 
        conz::MsgType::Normal);
    divider_ver();
    pprint_type!(
        &astr::from_str("relative:").pad_after(len_relative), 
        conz::MsgType::Normal);
    divider_ver();
    pprint_type!(
        &astr::from_str("time date:").pad_after(len_dt),
        conz::MsgType::Normal);
        divider_ver();
    pprint_type!(
        &astr::from_str("type:").pad_after(len_type),
        conz::MsgType::Normal);
    divider_ver_edge();
    pprintln!(&"");
    pprintln_type!(&divider_hor("-"), conz::MsgType::Highlight);
    let now = data::DT::new();
    for x in points{
        let diff = now.diff(&x.dt);
        let timecol = diff_color(&diff);
        divider_ver_edge();
        pprint_type!(
            &x.title.pad_after(len_title),
            conz::MsgType::Normal);
        divider_ver();
        pprint_type!(
            &diff.string_significant()
                .to_astr()
                .pad_after(len_relative),
            timecol);
        divider_ver();
        pprint_type!(
            &x.dt.str_datetime().concat(astr::from_str(" "))
                .concat(x.dt.str_dayname_short()).pad_after(len_dt),
            conz::MsgType::Value);
        divider_ver();
        pprint_type!(
            &x.ptype.to_astr().pad_after(len_type),
            conz::MsgType::Normal);
        divider_ver_edge();
        pprintln!(&"");
    }
    pprintln_type!(&divider_hor("="), conz::MsgType::Highlight);
}
