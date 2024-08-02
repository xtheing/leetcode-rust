use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut nodes_queue = VecDeque::new();
        let mut sums_queue = VecDeque::new();

        nodes_queue.push_back(root);
        sums_queue.push_back(0);

        while let Some(node) = nodes_queue.pop_front() {
            let current_sum = sums_queue.pop_front().unwrap();

            if let Some(node) = node {
                let node_ref = node.borrow();
                let new_sum = current_sum + node_ref.val;

                // 如果是叶子节点，检查路径总和是否等于目标值
                if node_ref.left.is_none() && node_ref.right.is_none() {
                    if new_sum == sum {
                        return true;
                    }
                } else {
                    // 如果不是叶子节点，则将其非空的子节点和新的路径总和加入队列
                    if node_ref.left.is_some() {
                        nodes_queue.push_back(node_ref.left.clone());
                        sums_queue.push_back(new_sum);
                    }
                    if node_ref.right.is_some() {
                        nodes_queue.push_back(node_ref.right.clone());
                        sums_queue.push_back(new_sum);
                    }
                }
            }
        }

        false
    }
}
