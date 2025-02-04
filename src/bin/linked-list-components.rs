use leetcode_rust::list_node::{vec_to_list, ListNode};

fn main() {
    println!(
        "{}",
        Solution::num_components(vec_to_list(vec![0, 1, 2, 3, 4]), vec![0, 3, 1, 4])
    );
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
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut bit_set: Vec<u8> = vec![0; 1251];
        let mut count = 0;
        for &x in nums.iter() {
            let bit_idx = x as usize >> 3;
            let bit_pos = x as usize & 7;
            bit_set[bit_idx] |= 1 << bit_pos;
        }
        let mut contains = false;
        let mut current = &head;
        while let Some(node) = current {
            let x = node.val;
            let bit_idx = x as usize >> 3;
            let bit_pos = x as usize & 7;
            if bit_set[bit_idx] & (1 << bit_pos) != 0 {
                if !contains {
                    count += 1;
                }
                contains = true;
            } else {
                contains = false;
            }
            current = &node.next;
        }
        count
    }
}
