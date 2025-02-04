fn main() {
    println!(
        "{:?}",
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3])
    );
    println!(
        "{:?}",
        Solution::smaller_numbers_than_current(vec![6, 5, 4, 8])
    );
    println!(
        "{:?}",
        Solution::smaller_numbers_than_current(vec![7, 7, 7, 7])
    );
}

struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = vec![0; 101];
        for &num in nums.iter() {
            freq[num as usize] += 1;
        }
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut prev = freq[0];
        freq[0] = 0;
        for i in 1..=100 {
            let temp = freq[i];
            freq[i] = freq[i - 1] + prev;
            prev = temp;
        }
        for i in 0..n {
            ans[i] = freq[nums[i] as usize];
        }
        ans
    }
}
