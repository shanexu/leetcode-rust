fn main() {
    assert_eq!(Solution::can_reach(String::from("011010"), 2, 3), true);
    assert_eq!(Solution::can_reach(String::from("01101110"), 2, 3), false);
    assert_eq!(Solution::can_reach(String::from("00"), 1, 1), true);
    assert_eq!(Solution::can_reach(String::from("00111010"), 3, 5), false);
}

struct Solution;

use std::collections::VecDeque;
use std::collections::HashSet;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let min_jump = min_jump as usize;
        let max_jump = max_jump as usize;
        let bs = s.as_bytes();
        let n = bs.len();

        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut visited = HashSet::new();
        visited.insert(0);
        let mut edge = 0;
        loop {
            match queue.pop_front() {
                None => break,
                Some(idx) => {
                    if idx == n - 1 {
                        return true;
                    }
                    let left = idx + min_jump;
                    let right = min(n - 1, idx + max_jump);
                    for i in max(edge+1, left)..=right {
                        if bs[i] == b'0' && !visited.contains(&i) {
                            visited.insert(i);
                            queue.push_back(i);
                        }
                    }
                    edge = right;
                }
            }
        }
        false
    }
}

#[inline]
fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}
