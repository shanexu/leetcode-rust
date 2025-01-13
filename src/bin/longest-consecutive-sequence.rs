fn main() {
    println!(
        "{}",
        Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2])
    );
    println!(
        "{}",
        Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
    );
}

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = nums.iter().map(|&v| v).collect();
        let mut ans = 0;
        for num in nums {
            if set.contains(&num) && !set.contains(&(num - 1)) {
                let mut curr = num;
                let mut count = 0;
                while set.contains(&curr) {
                    set.remove(&curr);
                    curr += 1;
                    count += 1;
                }
                ans = ans.max(count);
            }
        }
        ans
    }
}
