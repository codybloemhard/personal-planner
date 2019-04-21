use super::wizard;
use super::astr;
use super::data;
use super::misc::{UnwrapDefault};

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
    Single,
    Multiple,
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
    let mut more_than_one = false;
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
            more_than_one = false;
            vec.clear();
            vec.push(i);
        }
        else if curr_score == score{
            more_than_one = true;
            vec.push(i);
        }
    }
    if score == 0{
        return (MatchResult::None, vec);
    }
    if score > 0 && !more_than_one{
        return (MatchResult::Single, vec);
    }
    if more_than_one{
        return (MatchResult::Multiple, vec);
    }
    //should not be reachable
    return (MatchResult::None, vec);
}
