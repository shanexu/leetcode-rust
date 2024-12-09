use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {}

struct Solution {}

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
