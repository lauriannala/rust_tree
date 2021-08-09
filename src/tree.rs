
pub enum Tree<T> {
    Leaf(T),
    Fork(T, T),
}

impl<T> Tree<T> {
    pub fn left(&mut self) -> &mut T {
        match self {
            Self::Leaf(ref mut _leaf) => panic!("left - Should not be called on Leaf!"),
            Self::Fork(ref mut left, ref mut _right) => left
        }
    }

    pub fn right(&mut self) -> &mut T {
        match self {
            Self::Leaf(ref mut _leaf) => panic!("right - Should not be called on Leaf!"),
            Self::Fork(ref mut _left, ref mut right) => right
        }
    }

    pub fn leaf(&mut self) -> &mut T {
        match self {
            Self::Leaf(ref mut leaf) => leaf,
            Self::Fork(ref mut _left, ref mut _right) => panic!("leaf - Should not be called on Fork!")
        }
    }
}
