fn main() {
}

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn visit(node: Ref<TreeNode>, values: &mut Vec<i32>) {
            if let Some(ref left) = node.left {
                visit(left.borrow(), values);
            }
            values.push(node.val);
            if let Some(ref right) = node.right {
                visit(right.borrow(), values);
            }
        }

        let mut valuse = vec![];
        if let Some(ref node) = root {
            visit(node.borrow(), &mut valuse);
        }
        valuse
    }
}
