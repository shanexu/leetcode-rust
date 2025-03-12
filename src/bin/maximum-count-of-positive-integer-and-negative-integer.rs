fn main() {
    assert_eq!(3, Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]));
    assert_eq!(3, Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
    assert_eq!(4, Solution::maximum_count(vec![5, 20, 66, 1314]));
}

struct Solution;
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut pos = 0;
        let mut neg = 0;
        for num in nums {
            match num.signum() {
                1 => pos += 1,
                -1 => neg += 1,
                _ => (),
            }
        }
        pos.max(neg)
    }
}
