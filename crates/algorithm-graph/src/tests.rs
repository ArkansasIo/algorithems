#[cfg(test)]
mod tests {
    use super::*;
    use algorithm_core::Node;

    #[test]
    fn tree_depth_and_walk_work() {
        let mut root = Node::new(GraphNode::Input("x".into()));
        let mut branch = Node::new(GraphNode::Operation("square".into()));
        branch.add(Node::new(GraphNode::Output("y".into())));
        root.add(branch);
        assert_eq!(depth(&root), 3);
        let mut count = 0;
        walk(&root, &mut |_| count += 1);
        assert_eq!(count, 3);
    }
}
