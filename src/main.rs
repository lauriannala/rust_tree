use crate::tree::Tree;
mod tree;

fn main() {
    let tree = tree::Leaf(1);

    println!("Leaf value: {}", tree.leaf());
}
