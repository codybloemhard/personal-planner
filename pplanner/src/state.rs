use super::save;
use super::data;

pub struct State{
    pub deadlines: save::BufferFile<data::Deadline>,
}

impl State{
    pub fn new() -> Option<Self>{
        let path = save::get_data_dir_path(save::DEADLINE_DIR);
        if path.is_none() {return Option::None;}
        let path = path.unwrap();
        Option::Some(State{
            deadlines: save::BufferFile::new(path),
        })
    }

    pub fn is_clean(&self) -> bool{
        return self.deadlines.is_clean();
    }

    pub fn flush_files(&mut self) -> bool{
        let res_deadlines = self.deadlines.write();
        return res_deadlines;
    }
}
