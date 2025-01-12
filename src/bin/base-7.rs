fn main() {
    println!("{}", Solution::convert_to_base7(-7));
}

struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let sign = num.signum();
        let mut num = num.abs();
        let mut ans = vec![];
        while num > 0 {
            let r = num % 7;
            num /= 7;
            ans.push(b'0' + r as u8);
        }
        if sign < 0 {
            ans.push(b'-');
        } else if sign == 0 {
            ans.push(b'0');
        }
        ans.reverse();
        String::from_utf8(ans).unwrap()
    }
}
