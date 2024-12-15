#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn print_matrix(matrix: Vec<Vec<i32>>) {
  for row in matrix {
    for k in row {
      print!("{:3} ", k);
    }
    println!();
  }
}

pub fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
  let mut head = None;
  let mut current = &mut head;

  for &num in nums.iter() {
    *current = Some(Box::new(ListNode::new(num)));
    current = &mut current.as_mut().unwrap().next;
  }

  head
}
