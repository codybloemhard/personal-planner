type Astr = Vec<u8>;

pub fn new() -> Astr{
    return Vec::new();
}

pub fn from_string(s: String) -> Astr{
    let mut buffer: Vec<u8> = Vec::new();
    for ch in s.chars() {
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    return buffer;
}

pub fn from_str(s: &str) -> Astr{
    let mut buffer: Vec<u8> = Vec::new();
    for ch in s.chars(){
        if !ch.is_ascii() { continue; }
        let val: u8 = ch as u8;
        buffer.push(val);
    }
    return buffer;
}

pub fn clear(astr: &mut Astr){
    astr.clear();
}

pub fn to_string(astr: &Astr) -> String{
    let mut s: String = String::new();
    for ch in astr{
        s.push(*ch as char);
    }
    return s;
}

pub fn split(astr: &Astr, splitchars: &Astr) -> Vec<Astr>{
    fn splitnow(splits: &mut Vec<Astr>, current: &mut Astr, counter: &mut u32){
        splits.push(current.clone());
        current.clear();
        *counter = 0;
    }
    let mut splits: Vec<Astr> = Vec::new();
    let mut current: Astr = Vec::new();
    let mut counter: u32 = 0;
    for ch in astr{
        let mut hit: bool = false;
        for sp in splitchars{
            if *ch == *sp{
                hit = true;
                break;
            }
        }
        if hit{
            if counter == 0{
                continue;
            }
            splitnow(&mut splits, &mut current, &mut counter);
            continue;
        }
        counter += 1;
        current.push(*ch);
    }
    if counter > 0{
        splitnow(&mut splits, &mut current, &mut counter);
    }
    return splits;
}