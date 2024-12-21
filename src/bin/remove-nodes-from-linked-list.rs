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
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;

        let mut stack = vec![];
        while let Some(node) = current {
            let v = node.val;
            while !stack.is_empty() && stack[stack.len() - 1] < v {
                stack.pop();
            }
            stack.push(v);
            current = node.next;
        }
        vec_to_list(stack)
    }
}

#[inline]
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &val in vec.iter() {
        *current = Some(Box::new(ListNode::new(val)));
        current = &mut current.as_mut().unwrap().next;
    }

    head
}