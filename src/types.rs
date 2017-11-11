/// Like a Vec but must have one element
#[derive(Debug)]
pub struct Many1<T> {
    head: Box<T>,
    tail: Vec<T>,
}
