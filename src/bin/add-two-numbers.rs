fn main() {
    Solution::print_list(Solution::add_two_numbers(
        Solution::new_list(&[1, 4, 9]),
        Solution::new_list(&[1, 4, 9]),
    ));
    println!();
    Solution::print_list(Solution::add_two_numbers(
        Solution::new_list(&[9, 4, 1]),
        Solution::new_list(&[9, 4, 1]),
    ));
    println!();
    Solution::print_list(Solution::add_two_numbers(
        Solution::new_list(&[9, 9, 9]),
        Solution::new_list(&[1]),
    ));
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut current = &mut head;
        let mut n1 = &l1;
        let mut n2 = &l2;
        let mut carry: i32 = 0;
        loop {
            let s;
            match (n1, n2) {
                (Some(b1), Some(b2)) => {
                    s = b1.val + b2.val + carry;
                    n1 = &b1.next;
                    n2 = &b2.next;
                }
                (Some(b1), None) => {
                    s = b1.val + carry;
                    n1 = &b1.next;
                }
                (None, Some(b2)) => {
                    s = b2.val + carry;
                    n2 = &b2.next
                }
                (None, None) => break,
            }
            current.next = Some(Box::new(ListNode::new(s % 10)));
            carry = s / 10;
            current = current.next.as_mut().unwrap();
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)))
        }
        return head.next;
    }

    pub fn print_list(l: Option<Box<ListNode>>) {
        Solution::print_list_mut(&l)
    }

    pub fn print_list_mut(l: &Option<Box<ListNode>>) {
        let mut n = l;
        loop {
            match n {
                Some(b) => {
                    println!("{}", b.val);
                    n = &b.next;
                }
                None => break,
            }
        }
    }

    pub fn new_list(slice: &[i32]) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut current = &mut head;
        for ele in slice.iter() {
            current.next = Some(Box::new(ListNode::new(*ele)));
            current = current.next.as_mut().unwrap();
        }
        head.next
    }
}
