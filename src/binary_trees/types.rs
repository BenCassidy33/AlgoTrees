#[derive(Default, Debug)]
pub struct BinaryTree<T> {
    pub head: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}
