fn main() {
    println!("{}", Solution::count_symmetric_integers(1, 100));
}

struct Solution;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut ans = 0;
        for i in low..=high {
            let s = format!("{}", i);
            let s = s.as_bytes();
            let n = s.len();
            if n % 2 == 0 {
                let mut l = 0i32;
                let mut r = 0i32;
                for j in 0..(n / 2) {
                    l += s[j] as i32;
                    r += s[n / 2 + j] as i32;
                }
                if l == r {
                    ans += 1;
                }
            }
        }
        ans
    }
}
