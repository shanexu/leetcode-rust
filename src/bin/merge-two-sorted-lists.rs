fn main() {
    ListNode::print_list(Solution::merge_two_lists(
        ListNode::new_list(&[1, 2, 4]),
        ListNode::new_list(&[1, 3, 4]),
    ));
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
        ListNode::print_list_ref(&l)
    }

    pub fn print_list_ref(l: &Option<Box<ListNode>>) {
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

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut fake_head = ListNode::new(0);
        let mut cursor = &mut fake_head;
        let mut cursor1 = &l1;
        let mut cursor2 = &l2;
        loop {
            match (cursor1, cursor2) {
                (Some(v1), Some(v2)) => {
                    if v1.val < v2.val {
                        cursor.next = Some(Box::new(ListNode::new(v1.val)));
                        cursor = cursor.next.as_mut().unwrap();
                        cursor1 = &v1.next;
                    } else {
                        cursor.next = Some(Box::new(ListNode::new(v2.val)));
                        cursor = cursor.next.as_mut().unwrap();
                        cursor2 = &v2.next;
                    }
                }
                (Some(v1), None) => {
                    cursor.next = Some(Box::new(ListNode::new(v1.val)));
                    cursor = cursor.next.as_mut().unwrap();
                    cursor1 = &v1.next;
                }
                (None, Some(v2)) => {
                    cursor.next = Some(Box::new(ListNode::new(v2.val)));
                    cursor = cursor.next.as_mut().unwrap();
                    cursor2 = &v2.next;
                }
                (None, None) => break,
            }
        }
        fake_head.next
    }
}
