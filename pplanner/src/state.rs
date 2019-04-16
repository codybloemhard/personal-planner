use super::save;
use super::data;

pub struct State{
    pub points: save::BufferFile<data::Point>,
}

impl State{
    pub fn new() -> Option<Self>{
        let path = save::get_data_dir_path(save::POINT_DIR);
        if path.is_none() {return Option::None;}
        let path = path.unwrap();
        Option::Some(State{
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
