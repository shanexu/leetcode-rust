fn main() {}

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

struct Solution;

use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: Ref<TreeNode>, values: &mut Vec<i32>) {
            values.push(node.val);
            if let Some(ref left) = node.left {
                helper(left.borrow(), values);
            }
            if let Some(ref right) = node.right {
                helper(right.borrow(), values);
            }
        }

        let mut values = vec![];
        if let Some(node) = root {
            helper(node.borrow(), &mut values);
        }
        values
    }
}