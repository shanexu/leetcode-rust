use leetcode_rust::tree_node::TreeNode;

fn main() {
    assert!(Solution::is_balanced(TreeNode::from_string(
        "3,9,20,null,null,15,7"
    )));
    assert!(!Solution::is_balanced(TreeNode::from_string(
        "1,2,2,3,3,null,null,4,4"
    )))
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (ok, _) = depth(root, 0);
        return ok;
    }
}

fn depth(node: Option<Rc<RefCell<TreeNode>>>, d: i32) -> (bool, i32) {
    match node {
        None => (true, d - 1),
        Some(n) => {
            let (l, ld) = depth(n.borrow().left.clone(), d + 1);
            if !l {
                return (false, 0);
            }
            let (r, rd) = depth(n.borrow().right.clone(), d + 1);
            if !r {
                return (false, 0);
            }
            if (ld - rd).abs() > 1 {
                return (false, 0);
            }
            (true, ld.max(rd))
        }
    }
}
