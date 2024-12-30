fn main() {
    ListNode::print_list(Solution::remove_nth_from_end(
        ListNode::new_list(&[1, 2, 3, 4]),
        2,
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

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut cursor = &head;
        let mut slow_cursor = &head;
        let mut n = n;
        let mut result_fake_head = ListNode::new(0);
        let mut result_cursor = &mut result_fake_head;
        loop {
            match cursor {
                Some(v) => {
                    cursor = &v.next;
                    if n == 0 {
                        match slow_cursor {
                            Some(u) => {
                                result_cursor.next = Some(Box::new(ListNode::new(u.val)));
                                result_cursor = result_cursor.next.as_mut().unwrap();
                                slow_cursor = &u.next;
                            }
                            None => {}
                        }
                    } else {
                        n -= 1;
                    }
                }
                None => break,
            }
        }
        match slow_cursor {
            Some(u) => {
                slow_cursor = &u.next;
            }
            None => {}
        }
        loop {
            match slow_cursor {
                Some(u) => {
                    result_cursor.next = Some(Box::new(ListNode::new(u.val)));
                    result_cursor = result_cursor.next.as_mut().unwrap();
                    slow_cursor = &u.next;
                }
                None => break,
            }
        }
        result_fake_head.next
    }
}
