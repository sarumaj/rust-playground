use ::forest::Node;

fn main() {
    let root = Node::new(1);
    let child1 = Node::new(2);
    let child2 = Node::new(3);

    Node::append_child(&root, child1.clone());
    Node::append_child(&root, child2.clone());

    let child3 = Node::new(4);
    Node::append_child(&child1, child3.clone());

    // Recursive reference
    Node::append_child(&child1, root.clone());

    // Allowing the tree to be printed up to a depth of 20 (prevents infinite recursion)
    root.print_tree(Some(&root), Some(25));
}
