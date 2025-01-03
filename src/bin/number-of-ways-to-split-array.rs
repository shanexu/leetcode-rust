fn main() {
}

struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let sum = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut prefix_sum = 0i64;
        for i in 0..(nums.len()-1) {
            prefix_sum += nums[i] as i64;
            let last_sum = sum - prefix_sum;
            if prefix_sum >= last_sum {
                ans += 1;
            }
        }
        ans
    }
}
