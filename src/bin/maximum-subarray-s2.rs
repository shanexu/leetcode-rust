fn main() {
    println!(
        "{}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
    println!("{}", Solution::max_sub_array(vec![5, 4, -1, 7, 8]));
}

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut sum = nums[0];
        let mut min = 0.min(nums[0]);
        let n = nums.len();
        for i in 1..n {
            let num = nums[i];
            sum += num;
            ans = ans.max(sum - min);
            min = min.min(sum);
        }
        ans
    }
}
