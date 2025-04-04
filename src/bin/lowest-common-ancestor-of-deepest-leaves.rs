use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::lca_deepest_leaves(TreeNode::from_string("3,5,1,6,2,0,8,null,null,7,4"))
    );
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (n, _) = travel(root, 0);
        n
    }
}

fn travel(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    if node.is_none() {
        return (None, depth - 1);
    }
    let (l, dl) = travel(node.clone().unwrap().borrow().left.clone(), depth + 1);
    let (r, dr) = travel(node.clone().unwrap().borrow().right.clone(), depth + 1);
    if dl > dr {
        (l, dl)
    } else if dl < dr {
        (r, dr)
    } else {
        (node, dl)
    }
}
