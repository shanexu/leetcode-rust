fn main() {
    println!("{}", Solution::missing_number(vec![3, 0, 1]));
    println!("{}", Solution2::missing_number(vec![3, 0, 1]));
}

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        (1 + n) * n / 2 - sum
    }
}

struct Solution2;

impl Solution2 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut x = 0;
        let n = nums.len() as i32;
        for (i, &t) in nums.iter().enumerate() {
            x ^= t;
            x ^= i as i32;
        }
        x ^ n
    }
}
