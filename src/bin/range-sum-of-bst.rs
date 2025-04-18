use leetcode_rust::tree_node::TreeNode;
fn main() {
    println!(
        "{}",
        Solution::range_sum_bst(TreeNode::from_string("10,5,15,3,7,null,18"), 7, 15)
    );
    println!(
        "{}",
        Solution::range_sum_bst(TreeNode::from_string("10,5,15,3,7,13,18,1,null,6"), 6, 10)
    );
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
use std::cell::RefCell;
use std::rc::Rc;
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
