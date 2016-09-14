

#[derive(Debug)]
pub enum List<T> {
    Nil,
    Cons(T, Box<List<T>>),
}


impl<T> List<T> {
    pub fn new() -> List<T> {
        unimplemented!();
    }
}
