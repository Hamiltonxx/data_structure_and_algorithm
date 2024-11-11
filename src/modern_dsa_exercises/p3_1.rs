#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: Vec<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: TreeNode) {
        self.children.push(Box::new(child));
    }
}

fn print_tree(node: &TreeNode, depth: usize) {
    println!("{:indent$}{}", "", node.value, indent= depth * 2);
    // Rust's ownership rules. When there are no more children to process, the recursion ends for
    // that branch
    for child in &node.children {
        print_tree(child, depth + 1);
    }
}

fn double_values(node: &mut TreeNode) {
    node.value *= 2;
    for child in &mut node.children {
        double_values(child);
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    root.add_child(TreeNode::new(2));
    root.add_child(TreeNode::new(3));

    println!("Initial Tree:");
    print_tree(&root, 0);
    
    double_values(&mut root);

    println!("\nTree After Doubling Values:");
    print_tree(&root, 0);
}
