mod initalize {
    use crate::binary_trees::types::BinaryTree;

    pub fn create<T>(
        head: T,
        left: Option<Box<BinaryTree<T>>>,
        right: Option<Box<BinaryTree<T>>>,
    ) -> BinaryTree<T> {
        BinaryTree { head, left, right }
    }

    pub fn create_empty<T: Default>() -> BinaryTree<T> {
        BinaryTree {
            head: Default::default(),
            left: None,
            right: None,
        }
    }
}
