use leetcode_rust::tree_node::TreeNode;

fn main() {}

struct Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        if let Some(node) = root {
            stack.push(node.clone());
        }
        while !stack.is_empty() {
            let mut temp_stack = vec![];
            let mut max = i32::MIN;
            while let Some(node) = stack.pop() {
                max = std::cmp::max(max, node.borrow().val);
                if let Some(left) = node.borrow().left.clone() {
                    temp_stack.push(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    temp_stack.push(right);
                }
            }
            ans.push(max);
            stack = temp_stack;
        }

        ans
    }
}
