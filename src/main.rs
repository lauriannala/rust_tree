mod tree;
use crate::tree::Tree;

fn main() {
    let mut fork = Tree::Fork(1, 2);
    let mut leaf = Tree::Leaf(3);
    println!("Fork left: {}", fork.left());
    println!("Fork right: {}", fork.right());

    println!("Leaf: {}", leaf.leaf());
}
