use std::collections::HashSet;

use super::save;
use super::data;
use super::astr;

pub struct State{
    pub fset: HashSet<astr::Astr>,
    pub points: save::BufferFile<data::Point>,
    pub points_archive: save::ArchiveFile<data::Point>,
    pub todos_todos: save::BufferFile<data::Todo>,
    pub todos_long: save::BufferFile<data::Todo>,
    pub todos_idea: save::BufferFile<data::Todo>,
    pub todos_archive: save::ArchiveFile<data::TodoArchived>,
}

impl State{
    pub fn new() -> Option<Self>{
        let msg = "State::new(), file should be here";
        //main.rs should not continue if save::setup_config_dir fails
        //save::setup_config_dir fails should return false if not all files are there
        let points_path = save::get_data_dir_path(save::POINT_DIR).expect(msg);
        let points_archive_path = save::get_data_dir_path(save::POINT_ARCHIVE_DIR).expect(msg);
        let todo_todo_path = save::get_data_dir_path(save::TODO_TODO_DIR).expect(msg);
        let todo_long_path = save::get_data_dir_path(save::TODO_LONG_DIR).expect(msg);
        let todo_idea_path = save::get_data_dir_path(save::TODO_IDEA_DIR).expect(msg);
        let todo_archive_path = save::get_data_dir_path(save::TODO_ARCHIVE_DIR).expect(msg);
        Option::Some(State{
            fset: HashSet::new(),
            points: save::BufferFile::new(points_path),
            points_archive: save::ArchiveFile::new(points_archive_path),
            todos_todos: save::BufferFile::new(todo_todo_path),
            todos_long: save::BufferFile::new(todo_long_path),
            todos_idea: save::BufferFile::new(todo_idea_path),
            todos_archive: save::ArchiveFile::new(todo_archive_path),
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
