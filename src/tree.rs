
pub trait Tree<T> {
    fn left() -> &'static mut T;
    fn right() -> &'static mut T;
    fn leaf() -> &'static mut T;
}

pub fn run() {
    println!("Hello world!");
}
