use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::construct_maximum_binary_tree(vec![3, 2, 1, 6, 0, 5])
    );
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        for x in nums {
            let node = Rc::new(RefCell::new(TreeNode::new(x)));
            while !stack.is_empty() && stack[stack.len() - 1].borrow().val < x {
                let pop_node = stack.pop().unwrap();
                pop_node.borrow_mut().right = node.borrow_mut().left.take();
                node.borrow_mut().left = Some(pop_node);
            }
            stack.push(node);
        }

        let mut prev_node = stack[0].clone();
        for i in 1..stack.len() {
            let node = stack[i].clone();
            prev_node.borrow_mut().right = Some(node.clone());
            prev_node = node;
        }
        Some(stack[0].clone())
    }
}
