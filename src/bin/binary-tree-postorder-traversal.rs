use leetcode_rust::tree_node::TreeNode;

fn main() {
    let root = TreeNode::from_string("1,null,2,3");
    println!("{:?}", Solution::postorder_traversal(root));
    let root = TreeNode::from_string("1,null,2,3");
    println!("{:?}", Solution2::postorder_traversal(root));
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
