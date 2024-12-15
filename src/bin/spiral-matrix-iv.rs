use leetcode_rust::list_node::{print_matrix, vec_to_list, ListNode};

fn main() {
    let nums = vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0];
    let list = vec_to_list(nums);
    print_matrix(Solution::spiral_matrix(3, 5, list));
}

struct Solution;
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![-1; n as usize]; m as usize];
        let mut dx: i32 = 1;
        let mut dy: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut node = &head;
        for _ in 1..=m * n {
            if let Some(inner_node) = node {
                matrix[j as usize][i as usize] = inner_node.val;
                node = &inner_node.next;
            } else {
                break;
            }
            let ni = i + dx;
            let nj = j + dy;
            if ni == -1 || nj == -1 || ni == n || nj == m {
                (dx, dy) = (-dy, dx);
                i = i + dx;
                j = j + dy;
            } else if matrix[nj as usize][ni as usize] != -1 {
                (dx, dy) = (-dy, dx);
                i = i + dx;
                j = j + dy;
            } else {
                i = ni;
                j = nj;
            }
        }
        matrix
    }
}
