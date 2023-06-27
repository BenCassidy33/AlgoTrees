#[derive(Default, Debug)]
pub struct Tree<T> {
    pub head: T,
    pub children: Vec<Option<Tree<T>>>,
}
