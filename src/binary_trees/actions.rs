use crate::binary_trees::types::BinaryTree;

pub enum InsertionType<T> {
    Node(BinaryTree<T>),
    Empty,
}

pub enum DeletionDirection {
    Left,
    Right,
}

// dumb shit I don't understand (I mean, I kinda do):
/// checks for type equality
impl<T: Default + PartialEq> PartialEq for BinaryTree<T> {
    fn eq(&self, other: &BinaryTree<T>) -> bool {
        self.head == other.head && self.left == other.left && self.right == other.right
    }
}

impl<T: Default + PartialEq> BinaryTree<T>
where
    BinaryTree<T>: PartialEq,
{
    /// Inserts a value on the left side of the node. NOTE: the value must be of the same type as the tree.
    ///
    /// Example:
    pub fn insert_left(&mut self, item: InsertionType<T>) {
        match item {
            InsertionType::Node(node) => self.left = Some(Box::new(node)),
            InsertionType::Empty => {
                self.left = Some(Box::new(BinaryTree {
                    head: Default::default(),
                    left: None,
                    right: None,
                }))
            }
        }
    }

    pub fn insert_right(&mut self, item: InsertionType<T>) {
        match item {
            InsertionType::Node(node) => self.right = Some(Box::new(node)),
            InsertionType::Empty => {
                self.right = Some(Box::new(BinaryTree {
                    head: Default::default(),
                    left: None,
                    right: None,
                }))
            }
        }
    }

    pub fn remove(&mut self, direction: DeletionDirection) {
        match direction {
            DeletionDirection::Left => self.left = None,
            DeletionDirection::Right => self.right = None,
        }
    }

    pub fn remove_left(&mut self) {
        self.remove(DeletionDirection::Left);
    }

    pub fn remove_right(&mut self) {
        self.remove(DeletionDirection::Right);
    }

    pub fn get_left_node(&self) -> Option<&BinaryTree<T>> {
        match &self.left {
            Some(node) => Some(node),
            None => None,
        }
    }

    pub fn get_right_node(&self) -> Option<&BinaryTree<T>> {
        match &self.right {
            Some(node) => Some(node),
            None => None,
        }
    }

    pub fn get_left_head_val(&self) -> Option<&T> {
        match &self.left {
            Some(node) => Some(&node.head),
            None => None,
        }
    }

    pub fn get_right_head_val(&self) -> Option<&T> {
        match &self.right {
            Some(node) => Some(&node.head),
            None => None,
        }
    }

    pub fn get_head(&self) -> &T {
        // ;)
        &self.head
    }
}
