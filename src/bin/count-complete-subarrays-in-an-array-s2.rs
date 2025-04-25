fn main() {
    println!(
        "{}",
        Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2])
    );
}

struct Solution;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 2001];
        let mut k = 0;
        for &num in nums.iter() {
            let num = num as usize;
            freq[num] += 1;
            if freq[num] == 1 {
                k += 1;
            }
        }
        freq.fill(0);
        let mut left = 0;
        let mut right = 0;
        let n = nums.len();
        let mut ans = 0;
        let mut s = 0;
        while right < n {
            let num_right = nums[right] as usize;
            freq[num_right] += 1;
            if freq[num_right] == 1 {
                s += 1;
            }
            right += 1;
            while s == k {
                ans += n - right + 1;
                let num_left = nums[left] as usize;
                freq[num_left] -= 1;
                if freq[num_left] == 0 {
                    s -= 1;
                }
                left += 1;
            }
        }
        ans as _
    }
}
