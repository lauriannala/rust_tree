mod tree;
use crate::tree::Tree;

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
}
