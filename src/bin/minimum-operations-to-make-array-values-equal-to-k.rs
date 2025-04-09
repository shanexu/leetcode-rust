fn main() {
    assert_eq!(2, Solution::min_operations(vec![5, 2, 5, 4, 5], 2));
    assert_eq!(-1, Solution::min_operations(vec![2, 1, 2], 2));
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = vec![0; 101];
        let mut ans = 0;
        for num in nums {
            if num < k {
                return -1;
            }
            if num > k {
                let idx = num as usize;
                ans += freq[idx] ^ 1;
                freq[idx] = 1;
            }
        }
        return ans;
    }
}
