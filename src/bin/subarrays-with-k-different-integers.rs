fn main() {
    println!(
        "{}",
        Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2)
    );
}

struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        fn at_most_k(nums: &Vec<i32>, k: i32, freq: &mut Vec<i32>) -> i32 {
            let n = nums.len();
            freq.fill(0);
            let mut win_begin = 0usize;
            let mut win_end = 0usize;
            let mut distinct_count = 0;
            let mut ans = 0;
            while win_end < n {
                let num = nums[win_end] as usize;
                freq[num] += 1;
                if freq[num] == 1 {
                    distinct_count += 1;
                }
                while k < distinct_count {
                    let num_begin = nums[win_begin] as usize;
                    freq[num_begin] -= 1;
                    if freq[num_begin] == 0 {
                        distinct_count -= 1;
                    }
                    win_begin += 1;
                }
                ans += win_end - win_begin + 1;
                win_end += 1;
            }
            ans as i32
        }
        let mut freq = vec![0; nums.len() + 1];
        at_most_k(&nums, k, &mut freq) - at_most_k(&nums, k - 1, &mut freq)
    }
}
