use core::fmt;
use std::{fmt::Display, ops::Deref};

#[derive(PartialEq, Debug)]
pub enum Tree<T: Display> {
    Leaf(T),
    Fork(Box<Tree<T>>, Box<Tree<T>>)
}

impl<T: Display> Tree<T> {
    pub fn left(&mut self) -> &mut Box<Tree<T>> {
        match self {
            Self::Leaf(ref mut _leaf) => panic!("left - Should not be called on Leaf!"),
            Self::Fork(ref mut left, ref mut _right) => left
        }
    }

    pub fn right(&mut self) -> &mut Box<Tree<T>> {
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

impl<T: Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        fn build_str<T: Display>(tree: &Tree<T>) -> String {
            match tree {
                Tree::Leaf(leaf) => leaf.to_string(),
                Tree::Fork(left, right) => {
                    let mut result = "[".to_owned() + &build_str(left.deref());
                    result.push_str(", ");
                    result.push_str(&build_str(right.deref()));
                    result + "]"
                }
            }
        }

        write!(f, "{}", build_str(self))
    }
}
