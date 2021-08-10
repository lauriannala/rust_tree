mod tree;

use crate::tree::Tree;
use crate::tree::TreeVisitor;
use crate::tree::Visitor;
use crate::tree::ElementCount;

impl<T> ElementCount for &[T] {
    fn element_count(&self) -> usize {
        self.len()
    }
}

impl<T: ?Sized> ElementCount for Box<T> {
    fn element_count(&self) -> usize {
        1
    }
}

fn main() {
    let mut tree = Tree::Fork(
        Box::new(Tree::Fork(
            Box::new(Tree::Leaf(1)),
            Box::new(Tree::Leaf(2))
        )),
        Box::new(Tree::Leaf(3)),
    );

    *tree.left().right().leaf() = 3;

    assert_eq!(
        tree,
        Tree::Fork(
            Box::new(Tree::Fork(
                Box::new(Tree::Leaf(1)),
                Box::new(Tree::Leaf(3))
            )),
            Box::new(Tree::Leaf(3)),
    ));

    assert_eq!(tree.to_string(), "[[1, 3], 3]");
    Visitor.visit(&tree);
    assert_eq!(tree.element_count(), 3);

    let slice: &[i32] = &[1, 2, 3, 4];
    assert_eq!(slice.element_count(), 4);

    let boxed: Box<str> = "foo".into();
    assert_eq!(boxed.element_count(), 1);
}
