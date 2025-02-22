use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
    );
}

struct Solution;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for (i, &v) in inorder.iter().enumerate() {
            map.insert(v, i);
        }
        help(&inorder, &postorder, 0, 0, inorder.len(), &map)
    }
}

fn help(
    inorder: &Vec<i32>,
    postorder: &Vec<i32>,
    i_pos: usize,
    p_pos: usize,
    len: usize,
    map: &HashMap<i32, usize>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if len == 0 {
        return None;
    }
    let root_val = postorder[p_pos + len - 1];
    let mut root = TreeNode::new(root_val);
    let i = *map.get(&root_val).unwrap();
    let l_len = i - i_pos;
    let r_len = len - l_len - 1;
    root.left = help(inorder, postorder, i_pos, p_pos, l_len, &map);
    root.right = help(inorder, postorder, i + 1, p_pos + l_len, r_len, &map);
    Some(Rc::new(RefCell::new(root)))
}
