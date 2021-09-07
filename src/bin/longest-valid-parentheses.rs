fn main() {
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from("(()"))
    );
    println!(
        "{}",
        Solution::longest_valid_parentheses(String::from(")()())"))
    );
    println!("{}", Solution::longest_valid_parentheses(String::from("")));
}

struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() == 0 || s.len() == 1 {
            return 0;
        }
        let mut m: i32 = 0;
        let bs = s.as_bytes();
        for i in 0..bs.len() {
            let mut sum = parenthesis_i32(bs[i]);
            if sum > 0 {
                continue;
            }
            for j in (i + 1)..bs.len() {
                sum += parenthesis_i32(bs[j]);
                if sum > 0 {
                    break;
                } else if sum == 0 {
                    let l = (j - i + 1) as i32;
                    if m < l {
                        m = l;
                    }
                } else {
                    if sum + ((s.len() - j - 1) as i32) < 0 {
                        break;
                    }
                }
            }
        }
        m
    }
}

#[inline(always)]
fn parenthesis_i32(b: u8) -> i32 {
    if b == ('(' as u8) {
        -1
    } else {
        1
    }
}
