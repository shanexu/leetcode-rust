use leetcode_rust::tree_node::TreeNode;
fn main() {
    // 创建根节点为 2
    let root = Rc::new(RefCell::new(TreeNode::new(2)));

    // 创建左节点为 3
    let left_node = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(left_node);

    // 创建右节点为 1
    let right_node = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().right = Some(right_node);
    Solution::recover_tree(&mut Some(root));
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
use std::rc::Rc;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn visit(
            root: &Option<Rc<RefCell<TreeNode>>>,
            values: &mut Vec<Rc<RefCell<TreeNode>>>,
            targets: &mut Vec<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(n) = root {
                visit(&n.borrow().left, values, targets);
                if values.len() < 2 {
                    values.push(n.clone());
                } else {
                    values[0] = values[1].clone();
                    values[1] = n.clone();
                }
                if values.len() == 2 {
                    if values[0].borrow().val > values[1].borrow().val {
                        if targets.len() == 0 {
                            targets.push(values[0].clone());
                            targets.push(values[1].clone());
                        } else {
                            targets[1] = values[1].clone();
                            return;
                        }
                    }
                }
                visit(&n.borrow().right, values, targets);
            }
        }
        let mut targets = vec![];
        visit(root, &mut vec![], &mut targets);
        std::mem::swap(
            &mut targets[0].borrow_mut().val,
            &mut targets[1].borrow_mut().val,
        );
    }
}
