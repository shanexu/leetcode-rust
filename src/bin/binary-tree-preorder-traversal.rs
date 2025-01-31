use leetcode_rust::tree_node::TreeNode;

fn main() {
    let root = TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7]);
    let result = Solution::preorder_traversal(root);
    println!("{:?}", result);

    let root = TreeNode::from_vec(vec![1, 2, 3, 4, 5, 6, 7]);
    let result = Solution2::preorder_traversal(root);
    println!("{:?}", result);
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

struct Solution2;

impl Solution2 {
    fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values = Vec::new();
        let mut stack = Vec::new();

        if let Some(node) = root {
            stack.push(node);
        }
        while let Some(node) = stack.pop() {
            values.push(node.borrow().val);
            if let Some(right) = node.borrow().right.clone() {
                stack.push(right);
            }
            if let Some(left) = node.borrow().left.clone() {
                stack.push(left);
            }
        }
        values
    }
}
