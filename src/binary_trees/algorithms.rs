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
