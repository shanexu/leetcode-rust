use std::usize;

use leetcode_rust::vec_vec_i32;

fn main() {
    assert!(Solution::check_valid_cuts(
        5,
        vec_vec_i32![[1, 0, 5, 2], [0, 2, 2, 4], [3, 2, 5, 3], [0, 4, 4, 5]],
    ));
}

struct Solution;

impl Solution {
    pub fn check_valid_cuts(_: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
        check(&mut rectangles, 0, 2) || check(&mut rectangles, 1, 3)
    }
}

#[inline]
fn check(rectangles: &mut Vec<Vec<i32>>, p: usize, q: usize) -> bool {
    rectangles.sort_unstable_by(|a, b| a[p].cmp(&b[p]).then_with(|| a[q].cmp(&b[q])));
    let len = rectangles.len();
    let mut count = 0;
    let mut start = rectangles[0][p];
    let mut i = 0;
    let stop = rectangles[len - 1][q];
    while start <= stop && i < len {
        let r = &rectangles[i];
        let begin = r[p];
        let end = r[q];
        if begin - start >= 0 {
            count += 1;
        }
        if count >= 3 {
            return true;
        }
        start = end;
        i += 1;
        while i < len {
            let r = &rectangles[i];
            if r[p] >= start {
                break;
            }
            start = start.max(r[q]);
            i += 1
        }
    }
    false
}
