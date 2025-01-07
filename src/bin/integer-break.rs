fn main() {
    for i in 2..10 {
        println!("{}", Solution::integer_break(i));
    }
}

struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        let k = n / 3;
        let r = n - k * 3;
        if r == 0 {
            3i32.pow(k as u32)
        } else if r == 2 {
            3i32.pow(k as u32) * 2
        } else {
            3i32.pow((k - 1) as u32) * 2 * 2
        }
    }
}
