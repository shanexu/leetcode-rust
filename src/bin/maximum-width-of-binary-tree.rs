use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!("{}", Solution::width_of_binary_tree(TreeNode::from_string("1,3,2,5,3,null,9"))) ;
}

struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        if let Some(node) = root.clone() {
            let mut queue = VecDeque::new();
            let mut size = 1;
            node.borrow_mut().val = 1;
            queue.push_back((0, node));
            while !queue.is_empty() {
                let mut new_size = 0;
                let mut min_value = 0;
                let mut max_value = 0;
                for i in 0..size {
                    if let Some((v, node)) = queue.pop_front() {
                        if i == 0 {
                            min_value = v;
                        } else if i == size - 1 {
                            max_value = v;
                        }
                        if let Some(left) = node.borrow().left.clone() {
                            queue.push_back((v * 2, left));
                            new_size += 1;
                        }
                        if let Some(right) = node.borrow().right.clone() {
                            queue.push_back((v * 2 + 1, right));
                            new_size += 1;
                        }
                    }
                }
                size = new_size;
                ans = ans.max(max_value - min_value + 1);
            }
        }
        ans
    }
}
