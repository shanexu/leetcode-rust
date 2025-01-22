use leetcode_rust::tree_node::TreeNode;
fn main() {}

struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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
        for i in 0..ans.len() {
            if i & 1 == 1 {
                ans[i].reverse();
            }
        }
        ans
    }
}
