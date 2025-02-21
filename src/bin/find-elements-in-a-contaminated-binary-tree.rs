use leetcode_rust::tree_node::TreeNode;

fn main() {
    let find_elements = FindElements::new(TreeNode::from_string("1,null,1,1,null,1"));
    println!("{}", find_elements.find(2));
    println!("{}", find_elements.find(3));
    println!("{}", find_elements.find(4));
    println!("{}", find_elements.find(5));
}

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
struct FindElements {
    data: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut data = HashSet::new();
        if let Some(root) = root.clone() {
            data.insert(0);
            Self::visit(0, root, &mut data);
        }
        Self { data }
    }

    fn find(&self, target: i32) -> bool {
        self.data.contains(&target)
    }

    fn visit(value: i32, root: Rc<RefCell<TreeNode>>, data: &mut HashSet<i32>) {
        if let Some(left) = root.borrow().left.clone() {
            let value = value * 2 + 1;
            data.insert(value);
            Self::visit(value, left, data);
        }
        if let Some(right) = root.borrow().right.clone() {
            let value = value * 2 + 2;
            data.insert(value);
            Self::visit(value, right, data);
        }
    }
}
