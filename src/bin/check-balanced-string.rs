fn main() {
    println!("{}", Solution::is_balanced(String::from("1234")));
    println!("{}", Solution::is_balanced(String::from("24123")));
}

struct Solution {}

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let (odd, even, _) = num.bytes().fold((0, 0, false), |(odd, even, is_even), b| {
            if is_even {
                (odd, even + b - b'0', false)
            } else {
                (odd + b - b'0', even, true)
            }
        });
        odd == even
    }
}
