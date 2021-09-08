fn main() {
    println!("{}", Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
}

struct Solution {}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut closest_sum = 30003;
        for i in 0..(n - 2) {
            let mut l = i + 1;
            let mut r = n - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if abs(target - sum) < abs(target - closest_sum) {
                    closest_sum = sum;
                }
                if sum > target {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        closest_sum
    }
}

fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}
