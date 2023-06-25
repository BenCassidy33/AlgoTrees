use algotrees::binary_trees::{self, types::BinaryTree};

fn main() {
    let tree: BinaryTree<i32> = binary_trees::initialize::create(
        5,
        None,
        Some(Box::new(binary_trees::initialize::create(6, None, None))),
    );

    println!(
        "Tree Width {:?}, Tree Left: {}, Tree Right {}",
        tree.width(),
        tree.get_far_left(),
        tree.get_far_right()
    );
}
