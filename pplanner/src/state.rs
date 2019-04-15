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
            deadlines: 
                save::BufferFile::new(
                path, 
                save::BufferFileType::Deadlines),
        })
    }
}
