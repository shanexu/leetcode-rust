fn main() {
    let preorder = vec![8, 5, 1, 7, 10, 12];
    let root = Solution::bst_from_preorder(preorder);
    println!("{:?}", root);
}

use leetcode_rust::tree_node::TreeNode;

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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        stack.push(root.clone());
        for &x in preorder.iter().skip(1) {
            let mut prev = None;
            while !stack.is_empty() && stack[stack.len() - 1].borrow().val < x {
                prev = stack.pop();
            }
            let node = Rc::new(RefCell::new(TreeNode::new(x)));
            match prev {
                None => {
                    stack[stack.len() - 1].borrow_mut().left = Some(node.clone());
                }
                Some(prev_node) => {
                    prev_node.borrow_mut().right = Some(node.clone());
                }
            }
            stack.push(node);
        }
        Some(root)
    }
}
