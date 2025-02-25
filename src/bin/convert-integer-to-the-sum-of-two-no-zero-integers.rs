fn main() {
    println!("{:?}", Solution::get_no_zero_integers(11));
}

struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n {
            if check(i) && check(n - i) {
                return vec![i, n - i];
            }
        }
        unreachable!();
    }
}

#[inline]
fn check(mut n: i32) -> bool {
    while n > 0 {
        if n % 10 == 0 {
            return false;
        }
        n /= 10;
    }
    true
}
