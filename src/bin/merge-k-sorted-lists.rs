fn main() {
    let vs = vec![
        ListNode::new_list(&[]),
        ListNode::new_list(&[1, 3, 4]),
        ListNode::new_list(&[]),
    ];
    ListNode::print_list(Solution::merge_k_lists(vs));
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut lists = lists;
        let mut fake_head = ListNode::new(0);
        let mut cursor = &mut fake_head;
        binary_heap_from(&mut lists);
        while lists.len() > 0 && binary_heap_peek(&lists).is_none() {
            binary_heap_pop(&mut lists);
        }
        while lists.len() > 0 {
            let root = binary_heap_pop(&mut lists);
            cursor.next = Some(Box::new(ListNode::new(root.as_ref().unwrap().val)));
            cursor = cursor.next.as_mut().unwrap();
            if root.as_ref().unwrap().next.is_some() {
                binary_heap_insert(&mut lists, root.unwrap().next);
            }
        }
        fake_head.next
    }
}

fn binary_heap_from(xs: &mut Vec<Option<Box<ListNode>>>) {
    let heap_size = xs.len();
    for i in 0..heap_size {
        let mut i = i;
        while i != 0 && great_than(&xs[binary_heap_parent(i)], &xs[i]) {
            xs.swap(i, binary_heap_parent(i));
            i = binary_heap_parent(i);
        }
    }
}

fn binary_heap_insert(xs: &mut Vec<Option<Box<ListNode>>>, x: Option<Box<ListNode>>) {
    xs.push(x);
    let mut i = xs.len() - 1;
    while i != 0 && great_than(&xs[binary_heap_parent(i)], &xs[i]) {
        xs.swap(i, binary_heap_parent(i));
        i = binary_heap_parent(i);
    }
}

fn great_than(x: &Option<Box<ListNode>>, y: &Option<Box<ListNode>>) -> bool {
    if x.is_none() {
        false
    } else if y.is_none() {
        true
    } else {
        x.as_ref().unwrap().val > y.as_ref().unwrap().val
    }
}

fn less_than(x: &Option<Box<ListNode>>, y: &Option<Box<ListNode>>) -> bool {
    if x.is_none() {
        true
    } else if y.is_none() {
        false
    } else {
        x.as_ref().unwrap().val < y.as_ref().unwrap().val
    }
}

fn binary_heap_peek(xs: &Vec<Option<Box<ListNode>>>) -> &Option<Box<ListNode>> {
    &xs[0]
}

fn binary_heap_pop(xs: &mut Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let heap_size = xs.len();
    if heap_size == 1 {
        return xs.pop().unwrap();
    }
    xs.swap(0, heap_size - 1);
    let root = xs.pop().unwrap();
    binary_heap_heapify(0, xs);
    root
}

fn binary_heap_heapify(i: usize, xs: &mut Vec<Option<Box<ListNode>>>) {
    let heap_size = xs.len();
    let l = binary_heap_left(i);
    let r = binary_heap_right(i);
    let mut smallest = i;
    if l < heap_size && less_than(&xs[l], &xs[i]) {
        smallest = l;
    }
    if r < heap_size && less_than(&xs[r], &xs[smallest]) {
        smallest = r;
    }
    if smallest != i {
        xs.swap(i, smallest);
        binary_heap_heapify(smallest, xs);
    }
}

fn binary_heap_parent(i: usize) -> usize {
    (i - 1) / 2
}

fn binary_heap_left(i: usize) -> usize {
    2 * i + 1
}

fn binary_heap_right(i: usize) -> usize {
    2 * i + 2
}
