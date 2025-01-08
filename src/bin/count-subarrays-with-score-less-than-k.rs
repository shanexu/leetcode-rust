fn main() {
    println!("{}", Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10));
}

struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let n = nums.len();
        let mut ans = 0;
        let mut i = 0usize;
        let mut j = 0usize;
        let mut s = nums[0] as i64;
        while i < n && j < n {
            if k > s * (j - i + 1) as i64 {
                j += 1;
                if j >= i {
                    ans += j - i;
                }
                if j < n {
                    s += nums[j] as i64;
                }
            } else {
                s -= nums[i] as i64;
                i += 1;
            }
        }
        ans as i64
    }
}
