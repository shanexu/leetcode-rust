use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::prune_tree(TreeNode::from_string("1,null,0,0,1"))
    );
    println!(
        "{:?}",
        Solution::prune_tree(TreeNode::from_string("1,0,1,0,0,0,1"))
    );
    println!(
        "{:?}",
        Solution::prune_tree(TreeNode::from_string("1,1,0,1,1,0,1,0"))
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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(root: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
            let mut node = root.borrow_mut();
            if let Some(left) = node.left.take() {
                node.left = helper(left);
            }
            if let Some(right) = node.right.take() {
                node.right = helper(right);
            }
            if node.left.is_none() && node.right.is_none() && node.val == 0 {
                None
            } else {
                Some(root.clone())
            }
        }

        if let Some(root) = root {
            helper(root)
        } else {
            None
        }
    }
}
