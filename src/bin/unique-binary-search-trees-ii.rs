use leetcode_rust::tree_node::TreeNode;

fn main() {
    println!("{:?}", Solution::generate_trees(3));
    println!("{:?}", Solution::generate_trees(4));
}

struct Solution;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn helper(
            nums: &[i32],
            pair @ (i, j): (usize, usize),
            memo: &mut HashMap<(usize, usize), Vec<Option<Rc<RefCell<TreeNode>>>>>,
        ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if i == j {
                return vec![None];
            }
            if let Some(v) = memo.get(&pair) {
                return v.clone();
            }
            let mut res = vec![];
            for k in i..j {
                let res_pre = helper(nums, (i, k), memo);
                let res_post = helper(nums, (k + 1, j), memo);
                for p in 0..res_pre.len() {
                    for q in 0..res_post.len() {
                        let node = Rc::new(RefCell::new(TreeNode::new(nums[k])));
                        node.borrow_mut().left = res_pre[p].clone();
                        node.borrow_mut().right = res_post[q].clone();
                        res.push(Some(node));
                    }
                }
            }
            memo.insert(pair, res.clone());
            res
        }
        let nums = (1..=n).collect::<Vec<_>>();
        helper(&nums, (0, nums.len()), &mut HashMap::new())
    }
}
