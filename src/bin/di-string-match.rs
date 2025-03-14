fn main() {
    println!("{:?}", Solution::di_string_match("IDID".to_string()));
    println!("{:?}", Solution::di_string_match("DDDD".to_string()));
    println!("{:?}", Solution::di_string_match("IIII".to_string()));
    println!("{:?}", Solution::di_string_match("DDI".to_string()));
    println!("{:?}", Solution::di_string_match("".to_string()));
}

struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut low = 0;
        let mut high = s.len() as i32;
        let mut ans = Vec::with_capacity(s.len() + 1);
        for c in s.chars() {
            if c == 'I' {
                ans.push(low);
                low += 1;
            } else {
                ans.push(high);
                high -= 1;
            }
        }
        ans.push(low);
        ans
    }
}
