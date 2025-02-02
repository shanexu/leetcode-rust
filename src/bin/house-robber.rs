fn main() {
    let nums = vec![2, 3, 2];
    let ans = Solution::rob(nums);
    println!("{}", ans);
}

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans1 = 0; // 没抢
        let mut ans2 = nums[0]; // 抢了
        let mut current_ans1: i32;
        let mut current_ans2: i32;
        for i in 1..n {
            current_ans1 = ans1.max(ans2);
            current_ans2 = ans1 + nums[i];
            ans1 = current_ans1;
            ans2 = current_ans2;
        }
        ans1.max(ans2)
    }
}
