use crate::trees::types::Tree;

pub fn create<T>(head: T, children: Vec<Option<Tree<T>>>) -> Tree<T> {
    Tree { head, children }
}

pub fn create_empty<T>() -> Tree<T>
where
    T: Default,
{
    Tree::default()
}
