/// The `BinaryTree` struct is a binary tree that can be used to store
/// Data is layed out in a Option Box Stucture
/// Option: Because a node can be null/none
/// Box for recursive data structure
/// ```
///
/// use algotrees::binary_trees::{initialize, prelude::BinaryTree};
/// let tree: BinaryTree<i32> =
///     initialize::create(5, None, Some(Box::new(initialize::create(6, None, None))));
///
/// assert_eq!(
///     tree,
///     BinaryTree {
///         head: 5,
///         left: None,
///         right: Some(Box::new(BinaryTree {
///             head: 6,
///             left: None,
///             right: None
///         }))
///     }
/// )
/// ```
#[derive(Default, Debug)]
pub struct BinaryTree<T> {
    pub head: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}
