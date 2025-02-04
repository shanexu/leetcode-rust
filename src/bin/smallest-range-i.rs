fn main() {
    println!("{}", Solution::smallest_range_i(vec![1, 3, 6], 3));
}

struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for v in nums {
            min = min.min(v);
            max = max.max(v);
        }
        let range = max - min;
        if range <= 2 * k {
            0
        } else {
            range - 2 * k
        }
    }
}
