use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::reverse_odd_levels(TreeNode::from_string("0,1,2,0,0,0,0,1,1,1,1,2,2,2,2"))
    );
}

struct Solution;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let mut stack = vec![];
        stack.push(root.clone().unwrap());
        let mut odd = true;
        'outer: loop {
            let mut next_stack = vec![];
            while let Some(node) = stack.pop() {
                if node.borrow().left.is_none() {
                    break 'outer;
                }
                next_stack.push(node.borrow().left.clone().unwrap());
                next_stack.push(node.borrow().right.clone().unwrap());
            }
            if odd {
                for i in 0..next_stack.len() / 2 {
                    let vl = &mut next_stack[i].borrow_mut().val;
                    let vr = &mut next_stack[next_stack.len() - 1 - i].borrow_mut().val;
                    mem::swap(vl, vr);
                }
            }
            odd = !odd;
            stack = next_stack;
        }
        root
    }
}
