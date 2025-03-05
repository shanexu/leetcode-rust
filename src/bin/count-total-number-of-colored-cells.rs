fn main() {
    assert_eq!(1, Solution::colored_cells(1));
    assert_eq!(5, Solution::colored_cells(2)); // +4
    assert_eq!(13, Solution::colored_cells(3)); // +8
    assert_eq!(25, Solution::colored_cells(4)); // +12
    assert_eq!(41, Solution::colored_cells(5)); // +16
}

struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        2 * n * (n - 1) + 1
    }
}
