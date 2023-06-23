use algographs::algographs::binary_tree::actions::*;
use algographs::algographs::binary_tree::*;

fn main() {
    let mut graph: BinaryTree<i32> = BinaryTree::create_empty();
    graph.insert_right(InsertionType::Empty);

    println!("{:?}", graph);
}
