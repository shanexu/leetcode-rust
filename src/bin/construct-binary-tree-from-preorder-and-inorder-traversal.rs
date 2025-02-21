use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
    );
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        help(&preorder, &inorder)
    }
}

fn help(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.len() == 0 {
        return None;
    }
    let root_val = preorder[0];
    let mut root = TreeNode::new(root_val);
    let i = inorder.iter().position(|&val| val == root_val).unwrap();
    root.left = help(&preorder[1..i + 1], &inorder[0..i]);
    root.right = help(&preorder[i + 1..], &inorder[i + 1..]);
    Some(Rc::new(RefCell::new(root)))
}
