fn main() {
    println!("{}", Solution::nth_ugly_number(10));
    println!("{}", Solution::nth_ugly_number(1));
}

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut nums = Vec::with_capacity(n);
        nums.push(1);
        let mut s2 = 0usize;
        let mut s3 = 0usize;
        let mut s5 = 0usize;
        for _ in 1..n {
            let u2 = nums[s2] * 2;
            let u3 = nums[s3] * 3;
            let u5 = nums[s5] * 5;
            let u = u2.min(u3).min(u5);
            nums.push(u);
            if u2 == u {
                s2 += 1;
            }
            if u3 == u {
                s3 += 1;
            }
            if u5 == u {
                s5 += 1;
            }
        }
        nums[n - 1]
    }
}
