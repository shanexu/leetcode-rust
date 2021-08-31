fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::reverse(120));
    println!("{}", Solution::reverse(0));
    println!("{}", Solution::reverse(1534236469));
}

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x >= 0 { 1 } else { -1 };
        let mut x = x * sign;
        let mut s: i32 = 0;
        while x > 0 {
            let d = x % 10;
            x = x / 10;
            match s.checked_mul(10).and_then(|v| v.checked_add(d)) {
                None => return 0,
                Some(v) => s = v,
            }
        }
        s.checked_mul(sign).unwrap_or(0)
    }
}
