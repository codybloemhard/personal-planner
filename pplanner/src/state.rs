use super::save;

pub struct State{
    pub deadlines: save::BufferFile,
}

impl State{
    pub fn new() -> Self{
        State{
            deadlines: 
                save::BufferFile::new(
                save::DEADLINE_DIR, 
                save::BufferFileType::Deadlines),
        }
    }
}