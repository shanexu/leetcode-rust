fn main() {
    println!(
        "{}",
        Solution::min_size_subarray(
            vec![18, 3, 11, 19, 7, 16, 6, 7, 3, 6, 18, 9, 9, 1, 14, 17, 15, 14, 12, 10],
            7
        )
    );
}

struct Solution;

impl Solution {
    pub fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut prefix_sums = std::collections::HashMap::new();
        let mut s = 0;
        for (i, &v) in nums.iter().enumerate() {
            s += v;
            prefix_sums.insert(s, i);
        }
        s = 0;
        let mut suffix_sums = std::collections::HashMap::new();
        for (j, &v) in nums.iter().enumerate().rev() {
            s += v;
            suffix_sums.insert(s, j);
        }
        let nums_sum = s;
        if nums_sum == target {
            return n as i32;
        }
        let mut ans = i32::MAX;
        let quotient = target / nums_sum;
        let remainder = target - quotient * nums_sum;
        if remainder == 0 {
            ans = ans.min(quotient * n as i32);
        }
        for x in 0..2 {
            let k = quotient - x;
            if k < 0 {
                break;
            }
            let r = remainder + nums_sum * x;
            s = 0;
            for (i, &v) in nums.iter().enumerate() {
                s += v;
                if s == r {
                    ans = ans.min(k * n as i32 + i as i32 + 1);
                } else {
                    if let Some(&j) = prefix_sums.get(&(r + s)) {
                        ans = ans.min(k * n as i32 + (j - i) as i32);
                    }
                    if r > s {
                        if let Some(&j) = suffix_sums.get(&(r - s)) {
                            ans = ans.min(k * n as i32 + (n - j) as i32 + i as i32 + 1);
                        }
                    }
                }
            }
            if let Some(&j) = suffix_sums.get(&r) {
                ans = ans.min(k * n as i32 + (n - j) as i32);
            }
        }
        if ans == i32::MAX {
            ans = -1;
        }
        ans
    }
}
