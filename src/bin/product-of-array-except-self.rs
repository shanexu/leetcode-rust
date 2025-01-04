fn main() {
    Solution::product_except_self(vec![1, 2, 3, 4]);
}

struct Solution;

impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];
        let mut product = 1;
        for i in 1..n {
            product *= nums[i - 1];
            ans[i] = product;
        }
        product = 1;
        let mut prev = nums[n - 1];
        nums[n - 1] = 1;
        for i in (0..n - 1).rev() {
            product *= prev;
            prev = nums[i];
            nums[i] = product;
        }
        for i in 0..n {
            ans[i] *= nums[i];
        }
        ans
    }
}
