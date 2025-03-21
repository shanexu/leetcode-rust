use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::inorder_traversal(TreeNode::from_string("1,2,3,4,5,null,8,null,null,6,7,9"))
    );
    println!(
        "{:?}",
        Solution2::inorder_traversal(TreeNode::from_string("1,2,3,4,5,null,8,null,null,6,7,9"))
    );
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

struct Solution2;

impl Solution2 {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut values = vec![];
        let mut stack = vec![];
        let mut current = root;
        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }
            if let Some(node) = stack.pop() {
                values.push(node.borrow().val);
                current = node.borrow().right.clone();
            }
        }
        values
    }
}
