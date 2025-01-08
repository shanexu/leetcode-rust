use leetcode_rust::list_node::ListNode;

fn main() {

}

struct Solution;

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut prev = None;
        while let Some(mut current_node) = current {
            let next_node = current_node.next.take();
            current_node.next = prev.take();
            prev = Some(current_node);
            current = next_node;
        }
        prev
    }
}