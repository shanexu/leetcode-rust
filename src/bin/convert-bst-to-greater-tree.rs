use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::convert_bst(TreeNode::from_string(
            "4,1,6,0,2,5,7,null,null,null,3,null,null,null,8"
        ))
    );
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        visit(root.clone(), 0);
        return root;
    }
}

fn visit(node: Option<Rc<RefCell<TreeNode>>>, s: i32) -> i32 {
    if let Some(node) = node {
        if node.borrow().right.is_none() {
            node.borrow_mut().val += s;
        } else {
            let v = visit(node.borrow().right.clone(), s);
            node.borrow_mut().val += v;
        }
        if node.borrow().left.is_none() {
            return node.borrow().val;
        }
        return visit(node.borrow().left.clone(), node.borrow().val);
    }
    0
}
