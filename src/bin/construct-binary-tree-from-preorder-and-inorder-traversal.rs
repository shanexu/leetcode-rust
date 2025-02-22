use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
    );
}

struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for i in 0..preorder.len() {
            map.insert(inorder[i], i);
        }
        help(&preorder, &inorder, 0, 0, preorder.len(), &map)
    }
}

fn help(
    preorder: &[i32],
    inorder: &[i32],
    p_pos: usize,
    i_pos: usize,
    length: usize,
    map: &HashMap<i32, usize>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if length == 0 {
        return None;
    }
    let root_val = preorder[p_pos];
    let mut root = TreeNode::new(root_val);
    let i = *map.get(&root_val).unwrap();
    let l_len = i - i_pos;
    let r_len = length - l_len - 1;
    root.left = help(preorder, inorder, p_pos + 1, i_pos, l_len, map);
    root.right = help(preorder, inorder, p_pos + 1 + l_len, i + 1, r_len, map);
    Some(Rc::new(RefCell::new(root)))
}
