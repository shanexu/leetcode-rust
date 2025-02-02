use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{}",
        Solution::is_same_tree(
            TreeNode::from_string("1,2,3"),
            TreeNode::from_string("1,2,3")
        )
    );
    println!(
        "{}",
        Solution::is_same_tree(
            TreeNode::from_string("1,2"),
            TreeNode::from_string("1,null,2")
        )
    );
    println!(
        "{}",
        Solution::is_same_tree(
            TreeNode::from_string("1,2,1"),
            TreeNode::from_string("1,1,2")
        )
    );
}

struct Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn helper(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(_), None) => false,
                (None, Some(_)) => false,
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();
                    p.val == q.val
                        && helper(p.left.clone(), q.left.clone())
                        && helper(p.right.clone(), q.right.clone())
                }
            }
        }
        helper(p, q)
    }
}
