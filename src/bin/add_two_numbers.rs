fn main() {
    Solution::loop_list(Some(Box::new(ListNode::new(1))), Some(Box::new(ListNode::new(1))));
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {}

impl Solution {
    // pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // }
    pub fn loop_list(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) {
        let mut s1: Option<Box<ListNode>> = None;
        let mut s2: Option<Box<ListNode>> = None;
        let mut sc: *Option<Box<ListNode>> = None;
        s1 = l1;
        s2 = l2;
        loop {
            match (s1, s2) {
                (Some(b1), Some(b2)) => {
                    let v = b1.val + b2.val;
                    let st = Some(Box::new(ListNode::new(v % 10)));
                    if sc.is_some() {
                    } else {
                        sc =  &st;
                    }
                }
                _ => break,
            }
        }
    }
}
