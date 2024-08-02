use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// DFS Function Definitions
fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    dfs_helper(&node, &mut result);
    result
}

fn dfs_helper(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(node) = node {
        let node = node.borrow();
        result.push(node.val); // Preorder
        dfs_helper(&node.left, result);
        dfs_helper(&node.right, result);
    }
}

// BFS Function Definitions
fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    let mut queue = VecDeque::new();
    if let Some(root) = root {
        queue.push_back(root);
    }

    while let Some(node) = queue.pop_front() {
        let node = node.borrow();
        result.push(node.val);
        if let Some(left) = &node.left {
            queue.push_back(left.clone());
        }
        if let Some(right) = &node.right {
            queue.push_back(right.clone());
        }
    }

    result
}

fn main() {
    // 构建一个简单的二叉树
    //       1
    //      / \
    //     2   3
    //    / \
    //   4   5
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_left = Rc::new(RefCell::new(TreeNode::new(4)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(5)));

    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    left.borrow_mut().left = Some(left_left.clone());
    left.borrow_mut().right = Some(left_right.clone());

    let dfs_result = dfs(Some(root.clone()));
    println!("DFS: {:?}", dfs_result);

    let bfs_result = bfs(Some(root.clone()));
    println!("BFS: {:?}", bfs_result);
}
