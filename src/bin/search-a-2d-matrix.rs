fn main() {
    println!(
        "{}",
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        )
    );
}

struct Solution;

use std::cmp::Ordering::{Equal, Greater, Less};
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut size = n * m;
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;
            let mid_val = matrix[mid / m][mid % m];
            let cmp = mid_val.cmp(&target);
            left = if cmp == Less { mid + 1 } else { left };
            right = if cmp == Greater { mid } else { right };
            if cmp == Equal {
                return true;
            }
            size = right - left;
        }
        false
    }
}
