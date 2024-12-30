use leetcode_rust::tree_node::TreeNode;

fn main() {}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(_), None) | (None, Some(_)) => false,
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();
                    if p.val != q.val {
                        false
                    } else {
                        helper(p.left.clone(), q.right.clone())
                            && helper(p.right.clone(), q.left.clone())
                    }
                }
            }
        }

        if let Some(root) = root {
            helper(root.borrow().left.clone(), root.borrow().right.clone())
        } else {
            true
        }
    }
}
