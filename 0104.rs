use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut depth = 0; // 当前深度
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while queue.len() > 0 {
            let level_size = queue.len(); // 当前层的节点数
            for _ in 0..level_size {
                // 处理的是Option<Option<Rc<RefCell<TreeNode>>>>
                if let Some(node) = queue.pop_front().unwrap() {
                    let node = node.borrow(); // 获取节点引用
                    if let Some(left) = &node.left {
                        queue.push_back(Some(left.clone()));
                    }
                    if let Some(right) = &node.right {
                        queue.push_back(Some(right.clone()));
                    }
                }
            }
            depth += 1;
        }
        depth
    }
}

// 测试
fn main() {
    // 构建测试用的二叉树
    //      1
    //     / \
    //    2   3
    //   / \
    //  4   5
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    let left_left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let left_right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    if let Some(r) = root.as_ref() {
        r.borrow_mut().left = left;
        r.borrow_mut().right = right;

        if let Some(l) = r.borrow().left.as_ref() {
            l.borrow_mut().left = left_left;
            l.borrow_mut().right = left_right;
        }
    }

    // 调用 max_depth 函数
    let depth = Solution::max_depth(root);
    println!("The max depth of the tree is: {}", depth);
    // 期待输出: The max depth of the tree is: 3
}
