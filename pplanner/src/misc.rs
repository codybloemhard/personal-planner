pub fn is_sorted<T: PartialOrd>(vec: &Vec<T>) -> bool{
    let len = vec.len();
    if len <= 1 {return true;}
    if len == 2 {
        return vec[0] <= vec[1];
    }
    let mut last = &vec[0];
    for i in 1..len{
        if last > &vec[i] {
            return false;
        }
        last = &vec[i];
    }
    return true;
}
