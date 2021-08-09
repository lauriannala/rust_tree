
pub trait Tree<T> {
    fn leaf(self) -> T;
}

pub struct Leaf<T>(pub T);

impl<T> Tree<T> for Leaf<T> {
    fn leaf(self) -> T {
        self.0
    }
}
