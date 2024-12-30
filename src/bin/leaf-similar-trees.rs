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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn helper(node: Rc<RefCell<TreeNode>>, leaves: &mut Vec<i32>) {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                leaves.push(node.val);
                return;
            }
            if node.left.is_some() {
                helper(node.left.clone().unwrap(), leaves);
            }
            if node.right.is_some() {
                helper(node.right.clone().unwrap(), leaves);
            }
        }
        let mut leaves1 = vec![];
        let mut leaves2 = vec![];
        helper(root1.unwrap(), &mut leaves1);
        helper(root2.unwrap(), &mut leaves2);
        leaves1 == leaves2
    }
}
