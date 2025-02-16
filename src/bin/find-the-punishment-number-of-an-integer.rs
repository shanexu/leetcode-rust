fn main() {
    println!("{}", Solution::punishment_number(100));
}

struct Solution;

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            if i % 9 != 0 && i % 9 != 1 {
                continue;
            }
            let s = i * i;
            if help(s, i) {
                ans += s;
            }
        }
        ans
    }
}

fn help(mut n: i32, target: i32) -> bool {
    if n == target {
        return true;
    }
    let mut m = 0;
    let mut f = 1;
    while n > 0 {
        m += (n % 10) * f;
        f *= 10;
        n = n / 10;
        if target < m {
            break;
        }
        if help(n, target - m) {
            return true;
        }
    }
    return false;
}
