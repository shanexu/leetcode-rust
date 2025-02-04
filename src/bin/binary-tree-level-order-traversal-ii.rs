use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::level_order_bottom(TreeNode::from_string("3,9,20,null,null,15,7"))
    );
}

struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut ans = vec![];
        if let Some(node) = root {
            queue.push_back((0, node.clone()));
        }
        while let Some((i, node)) = queue.pop_front() {
            if ans.len() == i {
                ans.push(vec![]);
            }
            ans[i].push(node.borrow().val);
            if let Some(left) = node.borrow().left.clone() {
                queue.push_back((i + 1, left));
            }
            if let Some(right) = node.borrow().right.clone() {
                queue.push_back((i + 1, right));
            }
        }
        ans.reverse();
        ans
    }
}
