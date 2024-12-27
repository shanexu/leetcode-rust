fn main() {
    println!("{}", Solution::single_number(vec![1]));
}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for v in nums {
            ans ^= v;
        }
        ans
    }
}
