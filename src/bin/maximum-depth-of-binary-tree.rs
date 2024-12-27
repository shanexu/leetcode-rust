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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: Rc<RefCell<TreeNode>>, depth: i32) -> i32 {
            let mut max_depth: i32 = depth;
            if let Some(left) = node.borrow().left.clone() {
                max_depth = max_depth.max(helper(left, depth + 1));
            }
            if let Some(right) = node.borrow().right.clone() {
                max_depth = max_depth.max(helper(right, depth + 1))
            }
            max_depth
        }
        if let Some(node) = root {
            helper(node, 1)
        } else {
            0
        }
    }
}
