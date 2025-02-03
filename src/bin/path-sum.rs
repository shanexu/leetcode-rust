use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{}",
        Solution::has_path_sum(
            TreeNode::from_string("5,4,8,11,null,13,4,7,2,null,null,null,1"),
            22
        )
    );
}

struct Solution {}

use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(ref node) => visit(node.borrow(), 0, target_sum),
        }
    }
}

fn visit(node: Ref<TreeNode>, sum: i32, target_sum: i32) -> bool {
    let sum = sum + node.val;
    if sum == target_sum && node.left.is_none() && node.right.is_none() {
        return true;
    }
    if let Some(ref left) = node.left {
        if visit(left.borrow(), sum, target_sum) {
            return true;
        }
    }
    if let Some(ref right) = node.right {
        if visit(right.borrow(), sum, target_sum) {
            return true;
        }
    }
    return false;
}
