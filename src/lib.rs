pub mod prelude {
    pub use super::algographs::binary_tree;
    //pub use super::algographs::binary_tree::actions::InsertionType;
    //pub use super::algographs::binary_tree::initialize;
    pub use super::algographs::binary_tree::BinaryTree;
}

pub mod algographs {

    pub mod binary_tree {

        #[derive(Default, Debug)]
        pub struct BinaryTree<T> {
            pub head: T,
            pub left: Option<Box<BinaryTree<T>>>,
            pub right: Option<Box<BinaryTree<T>>>,
        }

        pub mod initialize {
            use super::BinaryTree;

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

        pub mod actions {
            use super::BinaryTree;

            pub enum InsertionType<T> {
                Node(BinaryTree<T>),
                Empty,
            }

            pub enum DeletionDirection {
                Left,
                Right,
            }

            // dumb shit I don't understand (I mean, I kinda do):
            impl<T: Default + PartialEq> PartialEq for BinaryTree<T> {
                fn eq(&self, other: &BinaryTree<T>) -> bool {
                    self.head == other.head && self.left == other.left && self.right == other.right
                }
            }

            impl<T: Default + PartialEq> BinaryTree<T>
            where
                BinaryTree<T>: PartialEq,
            {
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

                pub fn get_left(&self) -> Option<&BinaryTree<T>> {
                    match &self.left {
                        Some(node) => Some(node),
                        None => None,
                    }
                }

                pub fn get_right(&self) -> Option<&BinaryTree<T>> {
                    match &self.right {
                        Some(node) => Some(node),
                        None => None,
                    }
                }

                pub fn get_left_val(&self) -> Option<&T> {
                    match &self.left {
                        Some(node) => Some(&node.head),
                        None => None,
                    }
                }

                pub fn get_right_val(&self) -> Option<&T> {
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
        }

        pub mod algorithms {

            use super::BinaryTree;

            pub fn get_width<T>(tree: &BinaryTree<T>) -> T
            where
                T: std::ops::Add<Output = T>
                    + std::ops::Sub<Output = T>
                    + std::cmp::PartialOrd
                    + PartialEq
                    + Default
                    + Copy,
            {
                let left = get_left(tree);
                let right = get_right(tree);
                if left > right {
                    return left - right;
                } else {
                    return right - left;
                }
            }

            pub fn get_left<T>(tree: &BinaryTree<T>) -> T
            where
                T: std::ops::Add<Output = T> + PartialEq + Default + Copy,
            {
                if tree.left.is_none() {
                    return tree.head;
                } else {
                    return get_left(&tree.left.as_ref().unwrap());
                }
            }

            pub fn get_right<T>(tree: &BinaryTree<T>) -> T
            where
                T: std::ops::Add<Output = T> + PartialEq + Default + Copy,
            {
                if tree.right.is_none() {
                    return tree.head;
                } else {
                    return get_right(&tree.right.as_ref().unwrap());
                }
            }
        }
    }
}
