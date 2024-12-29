fn main() {
    println!("{}", Solution::min_add_to_make_valid("))((()()".to_string()));
}

struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open_count = 0;
        let mut close_count = 0;
        for c in s.chars() {
            if c == '(' {
                open_count += 1;
            }
            if c == ')' {
                if open_count > 0 {
                    open_count -= 1;
                } else {
                    close_count += 1;
                }
            }
        }
        open_count + close_count
    }
}