use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() || nums[0] == -1 {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        let mut i = 1;

        while i < nums.len() {
            let node = queue.pop_front().unwrap();
            let mut node_mut = node.borrow_mut();

            if i < nums.len() && nums[i] != -1 {
                let left = Rc::new(RefCell::new(TreeNode::new(nums[i])));
                node_mut.left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            if i < nums.len() && nums[i] != -1 {
                let right = Rc::new(RefCell::new(TreeNode::new(nums[i])));
                node_mut.right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    pub fn from_string(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let parse_result: Result<Vec<i32>, _> = s
            .split(',')
            .enumerate()
            .map(|(idx, token)| {
                let token = token.trim().to_lowercase();
                if token == "null" {
                    Ok(-1)
                } else {
                    token
                        .parse::<i32>()
                        .map_err(|_| format!("Invalid number at position {}", idx + 1))
                }
            })
            .collect();

        match parse_result {
            Ok(nums) => Self::from_vec(nums),
            Err(_) => None,
        }
    }
}
