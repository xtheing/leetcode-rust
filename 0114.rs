use std::cell::RefCell; // 提供内部可变性，允许在不可变借用中修改数据
use std::collections::VecDeque;
use std::rc::Rc; // 允许对TreeNode进行多重所有权共享 // VecDeque是一个双端队列，这里我们使用它作为栈

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>, // 使用Option包装左子节点，None表示没有子节点
    pub right: Option<Rc<RefCell<TreeNode>>>, // 同上，用于右子节点
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
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new(); // 用于存储遍历结果
        let mut stack = VecDeque::new(); // 使用VecDeque作为栈

        if let Some(node) = root {
            stack.push_back(node); // 将根节点压入栈中
        }

        while let Some(node) = stack.pop_back() {
            // 循环直到栈为空
            let node = node.borrow(); // 使用.borrow()来获取RefCell内部值的不可变引用
            result.push(node.val); // 将节点值添加到结果列表中
            if let Some(right) = node.right.clone() {
                // 如果存在右子节点
                stack.push_back(right); // 先将右子节点压入栈中
            }
            if let Some(left) = node.left.clone() {
                // 如果存在左子节点
                stack.push_back(left); // 然后将左子节点压入栈中
            }
        }
        result // 返回遍历结果
    }
}
