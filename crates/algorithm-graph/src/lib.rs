use algorithm_core::Node;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogicOp { And, Or, Not, Xor }

#[derive(Clone, Debug)]
pub enum GraphNode {
    Input(String),
    Operation(String),
    Branch { condition: String, if_true: String, if_false: String },
    Output(String),
}

pub type AlgorithmTree = Node<GraphNode>;

pub fn walk<T>(root: &Node<T>, visit: &mut impl FnMut(&T)) {
    visit(&root.value);
    for child in &root.children { walk(child, visit); }
}

pub fn depth<T>(root: &Node<T>) -> usize {
    root.children.iter().map(depth).max().unwrap_or(0) + 1
}
