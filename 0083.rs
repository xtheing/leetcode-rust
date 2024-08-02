// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = head.as_mut()?; // 这里给到的是 &mut Box<ListNode>类型
        while let Some(next) = node.next.as_mut() {
            // next 是一个Option<Box<ListNode>>类型
            if node.val == next.val {
                node.next = next.next.take(); // 这里返回的是一个Option<Box<ListNode>>类型
            } else {
                node = node.next.as_mut()?; // 这里给到的是一个&mut Box<ListNode>类型
            }
        }
        head
    }
}
