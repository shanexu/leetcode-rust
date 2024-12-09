// Definition for a binary tree node.

use std::cell::{RefCell, Ref};
use std::rc::Rc;

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

struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        match root {
            None => vec![],
            Some(ref node) => {
                let mut path = vec![];
                let mut results = vec![];
                visit(node.borrow(), &mut path, &mut results, 0, target_sum);
                results
            }
        }
    }
}

fn visit(node: Ref<TreeNode>, path: &mut Vec<i32>, results: &mut Vec<Vec<i32>>, sum: i32, target_sum: i32) {
    path.push(node.val);
    let sum = sum + node.val;
    if sum == target_sum && node.left.is_none() && node.right.is_none() {
        results.push(path.clone());
    }
    if let Some(ref left) = node.left {
        visit(left.borrow(), path, results, sum, target_sum);
    }
    if let Some(ref right) = node.right {
        visit(right.borrow(), path, results, sum, target_sum);
    }
    path.pop();
}
