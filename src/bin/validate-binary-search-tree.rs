use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!("{}", Solution::is_valid_bst(TreeNode::from_string("2,1,3")));
    println!(
        "{}",
        Solution::is_valid_bst(TreeNode::from_string("5,1,4,null,null,3,6"))
    );
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(root: Rc<RefCell<TreeNode>>, nums: &mut Vec<i32>) -> bool {
            let root = root.borrow();
            if let Some(left) = root.left.clone() {
                if !helper(left, nums) {
                    return false;
                }
            }
            if nums.len() < 2 {
                nums.push(root.val);
            } else {
                nums[0] = nums[1];
                nums[1] = root.val
            }
            if nums.len() == 2 {
                if nums[0] >= nums[1] {
                    return false;
                }
            }
            if let Some(right) = root.right.clone() {
                helper(right, nums)
            } else {
                true
            }
        }

        if let Some(root) = root {
            helper(root, &mut vec![])
        } else {
            true
        }
    }
}
