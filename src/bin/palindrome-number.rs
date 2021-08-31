fn main() {
    let xs: Vec<i32> = vec![121, -121, 10, -101, 0];
    for x in xs {
        println!("{}", Solution::is_palindrome(x))
    }
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }
        return x == reverse(x)
    }
}

fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut s: i32 = 0;
    while x > 0 {
        let d = x % 10;
        x = x / 10;
        match s.checked_mul(10).and_then(|v| v.checked_add(d)) {
            None => return 0,
            Some(v) => s = v,
        }
    }
    s
}
