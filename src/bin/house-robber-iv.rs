fn main() {
    assert_eq!(5, Solution::min_capability(vec![2, 3, 5, 9], 2));
}

struct Solution;

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut left = 1_000_000_000;
        let mut right = 0;
        for &num in nums.iter() {
            left = left.min(num);
            right = right.max(num);
        }
        'out: while left < right {
            let mid = (left + right) >> 1;
            let mut count = 0;
            let mut i = 0;
            while i < n {
                if nums[i] <= mid {
                    count += 1;
                    i += 2;
                    if count >= k {
                        right = mid;
                        continue 'out;
                    }
                } else {
                    i += 1;
                }
            }
            left = mid + 1;
        }
        left
    }
}
