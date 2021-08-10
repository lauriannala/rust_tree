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

pub trait TreeVisitor<T: Display> {
    fn visit(&mut self, content: &Tree<T>);
    fn visit_leaf(&mut self, content: &T);
    fn visit_fork(&mut self, left: &Box<Tree<T>>, right: &Box<Tree<T>>);
}

pub struct Visitor;
impl<T: Display> TreeVisitor<T> for Visitor {
    fn visit(&mut self, content: &Tree<T>) {
        match content {
            Tree::Leaf(leaf) => self.visit_leaf(leaf),
            Tree::Fork(left, right) => self.visit_fork(left, right),
        }
    }

    fn visit_leaf(&mut self, content: &T) {
        println!("Visiting leaf, content: {}", content);
    }

    fn visit_fork(&mut self, left: &Box<Tree<T>>, right: &Box<Tree<T>>) {
        println!("Visiting fork, left: {}, right: {}", left, right);
        self.visit(left.deref());
        self.visit(right.deref());
    }
}

pub trait ElementCount {
    fn element_count(&mut self) -> u32;
}

impl<T: Display> ElementCount for Tree<T> {
    fn element_count(&mut self) -> u32 {
        fn get_count<T: Display>(tree: &Tree<T>) -> u32 {
            match tree {
                Tree::Leaf(_) => 1,
                Tree::Fork(left, right) => get_count(left.deref()) + get_count(right.deref())
            }
        }

        get_count(self)
    }
}
