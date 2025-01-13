fn main() {
    println!(
        "{}",
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
    );
    println!(
        "{}",
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
    );
}

struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut ans = 0;
        for num in nums {
            if num == 0 {
                ans = ans.max(count);
                count = 0;
            } else {
                count += 1;
            }
        }
        ans = ans.max(count);
        ans
    }
}
