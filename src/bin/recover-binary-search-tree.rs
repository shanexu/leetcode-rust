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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn visit(
            root: &Option<Rc<RefCell<TreeNode>>>,
            values: &mut Vec<i32>,
            targets: &mut Vec<i32>,
        ) {
            if let Some(n) = root {
                let n = n.borrow();
                visit(&n.left, values, targets);
                if values.len() < 2 {
                    values.push(n.val);
                } else {
                    values[0] = values[1];
                    values[1] = n.val;
                }
                if values.len() == 2 {
                    if values[0] > values[1] {
                        if targets.len() == 0 {
                            targets.push(values[0]);
                            targets.push(values[1]);
                        } else {
                            targets[1] = values[1];
                            return;
                        }
                    }
                }
                visit(&n.right, values, targets);
            }
        }
        fn recover(root: &mut Option<Rc<RefCell<TreeNode>>>, targets: &Vec<i32>, count: &mut usize) {
            if *count == 2 {
                return;
            }
            if let Some(n) = root {
                let mut n = n.borrow_mut();
                recover(&mut n.left, targets, count);
                if *count == 0 && n.val == targets[0] {
                    n.val = targets[1];
                    *count += 1;
                } else if *count == 1 && n.val == targets[1] {
                    n.val = targets[0];
                    *count += 1;
                }
                recover(&mut n.right, targets, count);
            }
        }
        let mut targets = vec![];
        visit(root, &mut vec![], &mut targets);
        recover(root, &targets, &mut 0);
    }
}
