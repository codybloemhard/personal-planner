pub trait UnwrapDefault<T>{
    fn unwrap_default(res: Option<T>) -> T;
    fn replace_if_not_default(&mut self, new: T);
}

impl<T: Default + PartialEq> UnwrapDefault<T> for T{
    fn unwrap_default(res: Option<T>) -> T{
        if let Some(resv) = res{
            return resv;
        }
        T::default()
    }

    fn replace_if_not_default(&mut self, new: T){
        if new == Self::default() { return; }
        // std::mem::replace(self, new);
        *self = new;
    }
}

pub fn is_sorted<T: PartialOrd>(vec: &[T]) -> bool{
    let len = vec.len();
    if len <= 1 { return true; }
    if len == 2 {
        return vec[0] <= vec[1];
    }
    let mut last = &vec[0];
    for item in vec.iter().take(len).skip(1){
        if last > item {
            return false;
        }
        last = item;
    }
    true
}
