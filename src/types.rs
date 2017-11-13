/// Like a Vec but must have one element
#[derive(Debug, Clone)]
pub struct Many1<T> {
    head: Box<T>,
    tail: Vec<T>,
}
