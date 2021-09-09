fn main() {
    ListNode::print_list(Solution::swap_pairs(ListNode::new_list(&[1, 2, 3, 4])));
}

// Definition for singly-linked list.
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

    pub fn new_list(slice: &[i32]) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut current = &mut head;
        for ele in slice.iter() {
            current.next = Some(Box::new(ListNode::new(*ele)));
            current = current.next.as_mut().unwrap();
        }
        head.next
    }

    pub fn print_list(l: Option<Box<ListNode>>) {
        ListNode::print_list_mut(&l)
    }

    pub fn print_list_mut(l: &Option<Box<ListNode>>) {
        let mut n = l;
        loop {
            match n {
                Some(b) => {
                    print!("{}, ", b.val);
                    n = &b.next;
                }
                None => break,
            }
        }
        println!();
    }
}

struct Solution;

use std::mem::{replace, swap};

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cursor = &mut head;
        while cursor.is_some() && cursor.as_ref().unwrap().next.is_some() {
            let mut n2 = replace(&mut cursor.as_mut().unwrap().next, None);
            swap(
                &mut cursor.as_mut().unwrap().next,
                &mut n2.as_mut().unwrap().next,
            );
            swap(&mut n2.as_mut().unwrap().next, cursor);
            swap(cursor, &mut n2);
            cursor = &mut cursor.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
    }
}
