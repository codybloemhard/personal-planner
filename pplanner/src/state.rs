use std::collections::HashSet;

use super::save;
use super::data;
use super::astr;

pub struct State{
    pub fset: HashSet<astr::AstrVec>,
    pub points: save::BufferFile<data::Point>,
}

impl State{
    pub fn new() -> Option<Self>{
        let path = save::get_data_dir_path(save::POINT_DIR);
        if path.is_none() {return Option::None;}
        let path = path.unwrap();
        Option::Some(State{
            fset: HashSet::new(),
            points: save::BufferFile::new(path),
        })
    }

    pub fn is_clean(&self) -> bool{
        return self.points.is_clean();
    }

    pub fn flush_files(&mut self) -> bool{
        let res_points = self.points.write();
        return res_points;
    }
}
