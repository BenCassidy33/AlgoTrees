pub mod actions;
pub mod algorithms;
pub mod initialize;
pub mod types;

pub mod prelude {
    pub use crate::binary_trees::actions::*;
    //pub use crate::binary_trees::algorithms::{get_left, get_right, get_width};
    pub use crate::binary_trees::initialize::{create, create_empty};
    pub use crate::binary_trees::types::BinaryTree;
}
