use leetcode_rust::tree_node::TreeNode;

fn main() {
    let root = TreeNode::from_string("5,4,8,11,null,13,4,7,2,null,null,5,1");
    let target_sum = 22;
    let results = Solution::path_sum(root, target_sum);
    for result in results {
        println!("{:?}", result);
    }
}

struct Solution {}

use std::cell::{Ref, RefCell};
use std::rc::Rc;
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

fn visit(
    node: Ref<TreeNode>,
    path: &mut Vec<i32>,
    results: &mut Vec<Vec<i32>>,
    sum: i32,
    target_sum: i32,
) {
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
