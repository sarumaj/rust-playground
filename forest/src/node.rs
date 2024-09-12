use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
/// This struct represents a node in a tree.
pub struct Node {
    /// The value of the node.
    value: i32,
    /// The children of the node.
    children: RefCell<Vec<Rc<Node>>>,
    /// The parent of the node.
    parent: RefCell<Weak<Node>>,
}

impl Node {
    /// This function is used to create a new node with a given value.
    pub fn new(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }

    /// This function is used to append a child node to a parent node.
    /// It takes a reference to the parent node and the child node.
    /// It sets the parent of the child node to the parent node and
    /// appends the child node to the parent node's children vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use forest::Node;
    ///
    /// fn main() {
    ///    let root = Node::new(1);
    ///
    ///    let child1 = Node::new(2);
    ///    Node::append_child(&root, child1.clone());
    ///
    ///    let child2 = Node::new(3);
    ///    Node::append_child(&root, child2.clone());
    ///
    ///    let child3 = Node::new(4);
    ///    Node::append_child(&child1, child3.clone());
    /// }
    /// ```

    pub fn append_child(parent: &Rc<Node>, child: Rc<Node>) {
        child.parent.replace(Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }

    /// This function is used to print the tree starting from the given node.
    /// It takes the current node, the depth of the current node, the prefix string,
    /// a boolean indicating if the current node is the last child of its parent,
    /// an optional maximum depth to print up to, and an optional count of strong and weak references.
    /// For internal use, it recursively prints the tree starting from the given node.
    fn print(
        node: &Node,
        depth: usize,
        prefix: &str,
        last: bool,
        max: Option<usize>,
        count: Option<[usize; 2]>,
    ) {
        // If a maximum depth is specified and we have reached it, print a placeholder and return
        if let Some(max_depth) = max {
            if depth > max_depth {
                if last {
                    println!("{}{}", prefix, "└─ ...");
                } else {
                    println!("{}{}", prefix, "├─ ...");
                }
                return;
            }
        }

        // Print the current node with the correct prefix, ast, value, and reference counts
        println!(
            "{}{}{}{}",
            prefix,
            if depth > 0 {
                if last {
                    "└─ "
                } else {
                    "├─ "
                }
            } else {
                ""
            },
            node.value,
            if let Some([strong, weak]) = count {
                format!(" ({}, {})", strong, weak)
            } else {
                "".to_string()
            }
        );

        // Recursively print the children of the current node
        let children = node.children.borrow();
        for (i, child) in children.iter().enumerate() {
            // Determine the new prefix for the child node
            let new_prefix = format!(
                "{}{}",
                prefix,
                if depth > 0 {
                    if last {
                        "   "
                    } else {
                        "│  "
                    }
                } else {
                    ""
                }
            );

            // Print the child node
            Node::print(
                child.borrow(),
                depth + 1,
                &new_prefix,
                i == children.len() - 1,
                max,
                Some([Rc::strong_count(child), Rc::weak_count(child)]),
            );
        }
    }

    /// This function is used to print the tree starting from the given node.
    /// It takes an optional reference to the root node and an optional maximum recursion depth.
    /// If a root node is provided, it prints the tree starting from the root node.
    /// If no root node is provided, it prints the tree starting from the node it has been called on.
    /// The maximum recursion depth limits the number of levels printed to prevent infinite recursion.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use forest::Node;
    ///
    /// fn main() {
    ///    let root = Node::new(1);
    ///
    ///    root.print_tree(None, None);
    ///    // The same as the previous line, but explicitly specifying the root node
    ///    // will allow the tree to count strong and weak references.
    ///    root.print_tree(Some(&root), None);
    /// }
    /// ```
    pub fn print_tree(&self, root: Option<&Rc<Node>>, maximum_recursion_depth: Option<usize>) {
        if let Some(root) = root {
            Node::print(
                root,
                0,
                "",
                false,
                maximum_recursion_depth,
                Some([Rc::strong_count(root), Rc::weak_count(root)]),
            );
        } else {
            Node::print(self, 0, "", false, maximum_recursion_depth, None);
        }
    }
}
