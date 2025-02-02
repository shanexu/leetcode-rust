fn main() {
    println!("{}", Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]));
}

struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut j_opt: Option<usize> = None;
        for (i, &x) in nums.iter().enumerate() {
            if x == 0 {
                if j_opt.is_none() {
                    j_opt = Some(i);
                }
            } else {
                if let Some(j) = j_opt {
                    let t = i - j;
                    ans += ((t + 1) * t / 2) as i64;
                    j_opt = None;
                }
            }
        }
        if let Some(j) = j_opt {
            let t = nums.len() - j;
            ans += ((t + 1) * t / 2) as i64;
        }
        ans
    }
}
