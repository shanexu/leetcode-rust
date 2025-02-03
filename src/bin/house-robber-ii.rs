fn main() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![0]), 0);
    assert_eq!(Solution::rob(vec![1]), 1);
    assert_eq!(Solution::rob(vec![1, 2]), 2);
    assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 4]), 6);
}

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        rob(&nums[0..nums.len() - 1]).max(rob(&nums[1..]))
    }
}

fn rob(nums: &[i32]) -> i32 {
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
