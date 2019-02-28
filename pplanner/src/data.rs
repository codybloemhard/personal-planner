use chrono::prelude::*;

pub struct DT {
    dt: chrono::DateTime<Local>,
}

impl DT {
    pub fn new() -> DT {
        DT{
            dt: Local::now(),
        }
    }

    pub fn str_datetime(self) -> String{    
        return format!("{}", self.dt.format("%H:%M:%S %d-%m-%Y"));
    }

    pub fn add(&mut self, days: i64, months: i64, years: i64){
        self.dt = self.dt + chrono::Duration::days(days + (months * 30) + (years * 365));
    }
}
