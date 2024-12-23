use leetcode_rust::tree_node::TreeNode;

fn main() {
    // println!("{}", reorder(vec![4,3]));
    // println!("{}", reorder(vec![7,6,8,5]));
    // println!("{}", reorder(vec![9,10]));
    let mut root = Rc::new(RefCell::new(TreeNode::new(0)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.borrow_mut().left.clone().unwrap().borrow_mut().left =
        Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.borrow_mut().left.clone().unwrap().borrow_mut().right =
        Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.borrow_mut().right.clone().unwrap().borrow_mut().left =
        Some(Rc::new(RefCell::new(TreeNode::new(8))));
    root.borrow_mut().right.clone().unwrap().borrow_mut().right =
        Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .left
        .clone()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .right
        .clone()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(10))));
    println!("{}", Solution::minimum_operations(Some(root)));
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
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut count = 0;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        stack.push(root.clone().unwrap());
        loop {
            let mut next_stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
            for node in stack {
                let node = node.borrow();
                if node.left.is_some() {
                    next_stack.push(node.left.clone().unwrap());
                }
                if node.right.is_some() {
                    next_stack.push(node.right.clone().unwrap());
                }
            }
            if next_stack.is_empty() {
                break;
            }
            if next_stack.len() >= 2 {
                let values: Vec<i32> = next_stack.iter().map(|n| n.borrow().val).collect();
                count += reorder(values);
            }
            stack = next_stack;
        }
        count
    }
}

fn reorder(mut arr: Vec<i32>) -> i32 {
    let mut tree_map = std::collections::BTreeMap::new();
    for (i, &v) in arr.iter().enumerate() {
        tree_map.insert(v, i);
    }
    let mut count = 0;
    for i in 0..arr.len() {
        let (_, idx) = tree_map.pop_first().unwrap();
        if idx != i {
            let value = arr[i];
            arr.swap(i, idx);
            *tree_map.get_mut(&value).unwrap() = idx;
            count += 1;
        }
    }
    count
}
