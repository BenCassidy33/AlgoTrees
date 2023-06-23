#![allow(dead_code)]

pub mod prelude {
    pub use super::algographs::binary_tree::actions::InsertionType;
    pub use super::algographs::binary_tree::BinaryTree;
}

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
                    //;)
                    &self.head
                }
            }
        }

        pub mod search {
            use super::BinaryTree;

            enum Direction {
                Right(i32),
                Left(i32),
            }

            impl<T> BinaryTree<T> {
                //TODO:
                //pub fn directions(haystack: BinaryTree<T>, needle: T) -> Option<Vec<Direction>> {}
                //pub fn bfs(haystack: BinaryTree<T>) -> &T {}
            }
        }
    }
}
