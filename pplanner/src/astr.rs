type Achar = u8;

pub struct Astr{
    chars: Vec<Achar>,
}

impl Astr{
    pub fn new() -> Astr{
        Astr{
            chars: Vec::new(),
        }
    }

    pub fn from(s: String) -> Astr{
        let mut buffer: Vec<Achar> = Vec::new();
        for ch in s.chars() {
            if !ch.is_ascii() { continue; }
            let val: u8 = ch as u8;
            buffer.push(val);
        }
        Astr{
            chars: buffer,
        }
    }

    pub fn to_string(&mut self) -> String{
        let mut s: String = String::new();
        for ch in &self.chars{
            s.push(*ch as char);
        }
        return s;
    }
}
