use leetcode_rust::list_node::ListNode;

fn main() {}

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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut values = to_vec(head);
        let mut result: Vec<i32> = vec![0; values.len()];
        let mut stack: Vec<usize> = vec![];
        for (i, &v) in values.iter().enumerate() {
            while !stack.is_empty() && values[stack[stack.len() - 1]] < v {
                let j = stack.pop().unwrap();
                result[j] = v;
            }
            stack.push(i);
        }
        result
    }
}

#[inline]
fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;

    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }

    result
}
