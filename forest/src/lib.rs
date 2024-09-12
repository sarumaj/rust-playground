//! # Forest
//!
//! A simple tree data structure.
//!
//! ## Example
//!
//! ```rust
//! use forest::Node;
//!
//! fn main() {
//!    let root = Node::new(1);
//!
//!    let child1 = Node::new(2);
//!    Node::append_child(&root, child1.clone());
//!
//!    let child2 = Node::new(3);
//!    Node::append_child(&root, child2.clone());
//!
//!    let child3 = Node::new(4);
//!    Node::append_child(&child1, child3.clone());
//!
//!    root.print_tree(Some(&root), Some(25));
//! }
//! ```

mod node;

pub use node::Node;
