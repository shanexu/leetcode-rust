fn main() {
    println!("{}", Solution::count_symmetric_integers(1, 100));
}

struct Solution;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut ans = 0;
        let mut x = low;
        let mut range_low = 10;
        let mut range_high = 99;
        let mut half = 1;
        while x <= high {
            if x < range_low {
                x = range_low
            } else if x > range_high {
                range_low *= 100;
                range_high = range_high * 100 + 99;
                half += 1;
            } else {
                let (r, s1) = sum_digits(x, half);
                let (_, s2) = sum_digits(r, half);
                if s1 == s2 {
                    ans += 1;
                }
                x += 1;
            }
        }
        ans
    }
}

#[inline]
fn sum_digits(mut num: i32, mut n: usize) -> (i32, i32) {
    let mut s = 0;
    while n > 0 {
        s += num % 10;
        num /= 10;
        n -= 1;
    }
    (num, s)
}
