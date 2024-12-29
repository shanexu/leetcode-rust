fn main() {
    println!("{}", Solution::rotated_digits(2));
}

struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            let mut s = 0;
            let mut o = i;
            let mut f = 1;
            let mut has_2569 = false;
            while o > 0 {
                let d = (o % 7) as usize;
                if d == 2 || d == 3 || d == 4 || d == 6 {
                    has_2569 = true;
                }
                o /= 7;
                s += DIGITS[d] * f;
                f *= 10;
            }
            if s > n {
                break;
            }
            if has_2569 {
                ans += 1
            }
        }
        ans
    }
}

const DIGITS: &[i32] = &[0, 1, 2, 5, 6, 8, 9];
