#![allow(dead_code)]

pub mod algographs {
    pub mod binary_tree {

        #[derive(Default, Debug)]
        pub struct BinaryTree<T> {
            head: T,
            left: Option<Box<BinaryTree<T>>>,
            right: Option<Box<BinaryTree<T>>>,
        }

        mod initialize {
            use super::BinaryTree;

            impl<T: Default> BinaryTree<T> {
                pub fn create(
                    head: T,
                    left: Option<Box<BinaryTree<T>>>,
                    right: Option<Box<BinaryTree<T>>>,
                ) -> BinaryTree<T> {
                    BinaryTree { head, left, right }
                }

                pub fn create_empty() -> BinaryTree<T> {
                    BinaryTree {
                        head: Default::default(),
                        left: None,
                        right: None,
                    }
                }
            }
        }

        pub mod actions {
            use super::BinaryTree;

            pub enum InsertionType<T> {
                Node(BinaryTree<T>),
                Empty,
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
            }

            impl<T: Default + PartialEq> BinaryTree<T>
            where
                BinaryTree<T>: PartialEq,
            {
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
            }
        }
    }
}
