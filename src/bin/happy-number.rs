fn main() {
    println!("{:?}", Solution::is_happy(1));
    println!("{:?}", Solution::is_happy(19));
    println!("{:?}", Solution::is_happy(2));
}

struct Solution;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = std::collections::HashSet::new();
        while !set.contains(&n) {
            if n == 1 {
                return true;
            }
            set.insert(n);
            n = digit_square(n);
        }
        false
    }
}

#[inline(always)]
fn digit_square(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        let d = n % 10;
        ans += d * d;
        n /= 10;
    }
    ans
}