fn main() {
    println!("{}", 1 % 1);
}

struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut s = 0;
        for (i, &v) in nums.iter().enumerate() {
            if n % (i + 1) == 0 {
                s += v * v;
            }
        }
        s
    }
}
