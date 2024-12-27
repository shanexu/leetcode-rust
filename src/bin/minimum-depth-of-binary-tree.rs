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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: Rc<RefCell<TreeNode>>) -> i32 {
            let mut child_depth: i32 = i32::MAX;
            if let Some(left) = node.borrow().left.clone() {
                child_depth = child_depth.min(helper(left));
            }
            if let Some(right) = node.borrow().right.clone() {
                child_depth = child_depth.min(helper(right))
            }
            if child_depth == i32::MAX {
                1
            } else {
                child_depth + 1
            }
        }
        if let Some(node) = root {
            helper(node)
        } else {
            0
        }
    }
}
