use crate::binary_trees::types::BinaryTree;

impl<T> BinaryTree<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::cmp::PartialOrd
        + PartialEq
        + Default
        + Copy,
{
    pub fn width(&self) -> T {
        let left = self.get_far_left();
        let right = self.get_far_right();
        if left > right {
            return left - right;
        } else {
            return right - left;
        }
    }

    pub fn get_far_left(&self) -> T {
        if self.left.is_none() {
            return self.head;
        } else {
            return self.left.as_ref().unwrap().get_far_left();
        }
    }

    pub fn get_far_right(&self) -> T {
        if self.right.is_none() {
            return self.head;
        } else {
            return self.right.as_ref().unwrap().get_far_right();
        }
    }
}
