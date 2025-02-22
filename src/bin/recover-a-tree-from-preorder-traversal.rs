use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!(
        "{:?}",
        Solution::recover_from_preorder("1-2--3--4-5--6--7".to_string())
    );
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn help(depth: usize, mut ts: &[u8]) -> (Option<Rc<RefCell<TreeNode>>>, &[u8]) {
            if ts.len() <= depth {
                return (None, ts);
            }
            for i in 0..depth {
                if ts[i] != b'-' {
                    return (None, ts);
                }
            }
            if ts[depth] == b'-' {
                return (None, ts);
            }
            let mut i = depth + 1;
            let mut val = (ts[depth] - b'0') as i32;
            while i < ts.len() && ts[i] != b'-' {
                val *= 10;
                val += (ts[i] - b'0') as i32;
                i += 1;
            }
            let mut root = TreeNode::new(val);
            (root.left, ts) = help(depth + 1, &ts[i..]);
            if root.left.is_none() {
                return (Some(Rc::new(RefCell::new(root))), ts);
            }
            (root.right, ts) = help(depth + 1, ts);
            (Some(Rc::new(RefCell::new(root))), ts)
        }
        let traversal = traversal.as_bytes();
        help(0, &traversal).0
    }
}
