use super::save;

pub struct State{
    pub deadlines: save::BufferFile,
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
