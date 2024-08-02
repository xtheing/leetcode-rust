// 链表节点的定义
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 合并两个有序链表的函数
pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // 使用 `Box::new(ListNode::new(0))` 创建一个哨兵节点
    let mut sentinel = Box::new(ListNode::new(0));
    let mut current = &mut sentinel;

    // 使用两个可变引用来遍历两个链表
    let mut list1 = l1;
    let mut list2 = l2;

    while list1.is_some() && list2.is_some() {
        let next_node = if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            let next = list1.as_mut().unwrap().next.take();
            std::mem::replace(&mut list1, next)
        } else {
            let next = list2.as_mut().unwrap().next.take();
            std::mem::replace(&mut list2, next)
        };

        // 将较小的节点连接到新链表的末尾
        current.next = next_node;

        // 移动 current 到新链表的末尾
        current = current.next.as_mut().unwrap();
    }

    // 连接剩余的链表部分
    current.next = if list1.is_some() { list1 } else { list2 };

    // 返回合并后的链表，跳过哨兵节点
    sentinel.next
}

fn main() {
    // 创建两个有序链表
    let mut list1 = Some(Box::new(ListNode::new(1)));
    list1.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    list1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut list2 = Some(Box::new(ListNode::new(1)));
    list2.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    list2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
}
