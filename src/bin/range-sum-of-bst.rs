use leetcode_rust::tree_node::TreeNode;
fn main() {

}

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        fn helper(node: Rc<RefCell<TreeNode>>, low: i32, high: i32) -> i32 {
            let node = node.borrow();
            let mut ans = 0;
            if node.val < high && node.right.is_some() {
                ans += helper(node.right.clone().unwrap(), low, high);
            }
            if node.val > low && node.left.is_some() {
                ans += helper(node.left.clone().unwrap(), low, high);
            }
            if node.val >= low && node.val <= high {
                ans += node.val;
            }
            ans
        }
        if let Some(root) = root {
            helper(root, low, high)
        } else {
            0
        }
    }
}