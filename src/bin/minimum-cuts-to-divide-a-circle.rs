fn main() {
    assert_eq!(0, Solution::number_of_cuts(1));
    assert_eq!(1, Solution::number_of_cuts(2));
    assert_eq!(3, Solution::number_of_cuts(3));
    assert_eq!(3, Solution::number_of_cuts(6));
    assert_eq!(50, Solution::number_of_cuts(100));
}

struct Solution;

impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if n % 2 == 0 {
            return n / 2;
        }
        n
    }
}
