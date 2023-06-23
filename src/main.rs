use algographs::prelude::{binary_tree::initialize::create, *};

fn main() {
    type J = u32;

    let graph: BinaryTree<J> = create(
        1,
        Some(Box::new(create(
            3,
            Some(Box::new(create(5, None, None))),
            Some(Box::new(create(3, None, None))),
        ))),
        Some(Box::new(create(
            2,
            None,
            Some(Box::new(create(9, None, None))),
        ))),
    );

    println!("{}", binary_tree::algorithms::get_width(&graph));
}
