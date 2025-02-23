use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1])
    );
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        help(&preorder, &postorder, 0, 0, preorder.len())
    }
}

fn help(
    preorder: &Vec<i32>,
    postorder: &Vec<i32>,
    pre_pos: usize,
    post_pos: usize,
    len: usize,
) -> Option<Rc<RefCell<TreeNode>>> {
    if len == 0 {
        return None;
    }
    let root_val = preorder[pre_pos];
    let mut root = TreeNode::new(root_val);
    if len == 1 {
        return Some(Rc::new(RefCell::new(root)));
    }
    if preorder[pre_pos + 1] == postorder[post_pos + len - 2] {
        root.left = help(preorder, postorder, pre_pos + 1, post_pos, len - 1);
        return Some(Rc::new(RefCell::new(root)));
    }
    let child_val = preorder[pre_pos + 1];
    for i in post_pos..(post_pos + len - 1) {
        if postorder[i] == child_val {
            let l_len = i - post_pos + 1;
            let r_len = len - l_len - 1;
            root.left = help(preorder, postorder, pre_pos + 1, post_pos, l_len);
            root.right = help(
                preorder,
                postorder,
                pre_pos + 1 + l_len,
                post_pos + l_len,
                r_len,
            );
            return Some(Rc::new(RefCell::new(root)));
        }
    }
    unreachable!()
}
