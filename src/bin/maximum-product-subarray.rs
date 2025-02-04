fn main() {
    println!("{}", Solution::max_product(vec![2, 3, -2, 4]));
    println!("{}", Solution::max_product(vec![-2, 0, -1]));
}

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = i32::MIN;
        let mut left = 0;
        let mut right = 0;
        for i in 0..n {
            if left == 0 {
                left = 1;
            }
            if right == 0 {
                right = 1;
            }
            left *= nums[i];
            right *= nums[n - i - 1];
            ans = ans.max(left).max(right);
        }
        ans
    }
}
