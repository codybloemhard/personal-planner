use std::collections::HashSet;

use super::save;
use super::data;
use super::astr;

pub struct State{
    pub fset: HashSet<astr::Astr>,
    pub points: save::BufferFile<data::Point>,
    pub points_archive: save::ArchiveFile<data::Point>,
}

impl State{
    pub fn new() -> Option<Self>{
        let msg = "State::new(), file should be here";
        //main.rs should not continue if save::setup_config_dir fails
        //save::setup_config_dir fails should return false if not all files are there
        let points_path = save::get_data_dir_path(save::POINT_DIR).expect(msg);
        let points_archive_path = save::get_data_dir_path(save::POINT_ARCHIVE_DIR).expect(msg);
        Option::Some(State{
            fset: HashSet::new(),
            points: save::BufferFile::new(points_path),
            points_archive: save::ArchiveFile::new(points_archive_path),
        })
    }

    pub fn is_clean(&self) -> bool{
        self.points.is_clean()
        && self.points_archive.is_clean()
    }

    pub fn flush_files(&mut self) -> bool{
        self.points.write()
    }
}
