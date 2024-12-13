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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: Ref<TreeNode>, values: &mut Vec<i32>) {
            if let Some(ref left) = node.left {
                helper(left.borrow(), values);
            }
            if let Some(ref right) = node.right {
                helper(right.borrow(), values);
            }
            values.push(node.val);
        }

        let mut values = vec![];
        if let Some(node) = root {
            helper(node.borrow(), &mut values);
        }
        values
    }
}

struct Solution2;

use std::collections::VecDeque;
impl Solution2 {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values = vec![];
        let mut stack1 = vec![];
        let mut stack2 = vec![];

        if let Some(node) = root {
            stack1.push(node);
        }

        while let Some(node) = stack1.pop() {
            stack2.push(node.borrow().val);
            if let Some(left) = node.borrow().left.clone() {
                stack1.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                stack1.push(right);
            }
        }

        while let Some(value) = stack2.pop() {
            values.push(value);
        }

        values
    }
}