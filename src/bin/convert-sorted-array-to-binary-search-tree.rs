use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
    );
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        help(&nums)
    }
}

pub fn help(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    let mid = nums.len() / 2;
    let mut t = TreeNode::new(nums[mid]);
    t.left = help(&nums[..mid]);
    t.right = help(&nums[mid + 1..]);
    Some(Rc::new(RefCell::new(t)))
}
