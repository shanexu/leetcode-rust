fn main() {
    println!("{}", Solution::title_to_number("Z".to_string()));
}

struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut number = 0;
        for &c in column_title.as_bytes() {
            number *= 26;
            number += (c - b'A' + 1) as i32;
        }
        number
    }
}
