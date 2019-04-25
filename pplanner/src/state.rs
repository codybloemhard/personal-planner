use std::collections::HashSet;

use super::save;
use super::data;
use super::astr;

pub struct State{
    pub fset: HashSet<astr::Astr>,
    pub points: save::BufferFile<data::Point>,
    pub points_archive: save::BufferFile<data::Point>,
}

impl State{
    pub fn new() -> Option<Self>{
        let points_path = save::get_data_dir_path(save::POINT_DIR);
        if points_path.is_none() {return Option::None;}
        let points_path = points_path.unwrap();
        let points_archive_path = save::get_data_dir_path(save::POINT_ARCHIVE_DIR);
        if points_archive_path.is_none() {return Option::None;}
        let points_archive_path = points_archive_path.unwrap();
        Option::Some(State{
            fset: HashSet::new(),
            points: save::BufferFile::new(points_path),
            points_archive: save::BufferFile::new(points_archive_path),
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
