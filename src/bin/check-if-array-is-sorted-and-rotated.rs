fn main() {
    println!("{}", Solution::check(vec![5, 1, 2, 3, 4]));
    println!("{}", Solution::check(vec![3, 4, 5, 1, 2]));
    println!("{}", Solution::check(vec![2, 1, 3, 4]));
    println!("{}", Solution::check(vec![1, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut prev = nums[0];
        let mut rotated = false;
        for &num in nums.iter().skip(1) {
            if num < prev {
                if rotated {
                    return false;
                }
                rotated = true;
            }
            prev = num;
        }
        if !rotated {
            true
        } else {
            nums[0] >= nums[nums.len() - 1]
        }
    }
}
