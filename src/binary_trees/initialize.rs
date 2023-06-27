use crate::binary_trees::types::BinaryTree;

/// The create function is for creating a binary tree with a head, left, and right
///
/// Example:
///
/// ```
/// use algotrees::binary_trees::{initialize, prelude::BinaryTree};
///
/// let tree: BinaryTree<i32> = initialize::create(
///     5i32,
///     Some(Box::new(initialize::create(3, None, None))),
///     Some(Box::new(initialize::create(7, None, None))),
/// );
///
/// assert_eq!(
///     tree,
///     BinaryTree {
///         head: 5,
///         left: Some(Box::new(BinaryTree {
///             head: 3,
///             left: None,
///             right: None
///         })),
///         right: Some(Box::new(BinaryTree {
///             head: 7,
///             left: None,
///             right: None
///         }))
///     }
/// )
///
/// ```

pub fn create<T>(
    head: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
) -> BinaryTree<T> {
    BinaryTree { head, left, right }
}

/*
pub fn from<T>(mut vec: Vec<Vec<T>>) -> BinaryTree<T>
where
    T: Default + Copy,
{
    // [1, 2, 3, 5, 7, 9] -> [[1], [2, 3], [5, 7], [9, None]]
    //                  1
    //                /   \
    //               2     3
    //              / \   / \
    //             5   7 9  None
}
*/

/// Creates an empty binary tree
/// Will create the head with the default value of the type ie. 0 for i32
///
/// Example:
///
/// ```
/// use algotrees::binary_trees::{initialize, prelude::BinaryTree};
///
/// let tree: BinaryTree<i32> = initialize::create_empty();
///
/// assert_eq!(
///     tree,
///     BinaryTree {
///         head: 0,
///         left: None,
///         right: None
///     }
/// )
/// ```

pub fn create_empty<T: Default>() -> BinaryTree<T> {
    BinaryTree::default()
}
