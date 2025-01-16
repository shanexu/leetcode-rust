fn main() {
    println!("{}", Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
    println!("{}", Solution::min_sub_array_len(4, vec![1, 4, 4]));
    println!(
        "{}",
        Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1])
    );
}

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut win_begin = 0;
        let mut win_end = 0; // 不包含
        let mut win_sum = 0;
        let n = nums.len();
        let mut ans = n + 1;

        while win_end < n {
            if win_sum < target {
                win_sum += nums[win_end];
                win_end += 1;
            }
            while win_begin < win_end && win_sum >= target {
                ans = ans.min(win_end - win_begin);
                win_sum -= nums[win_begin];
                win_begin += 1;
            }
        }

        if ans == n + 1 {
            0
        } else {
            ans as i32
        }
    }
}
