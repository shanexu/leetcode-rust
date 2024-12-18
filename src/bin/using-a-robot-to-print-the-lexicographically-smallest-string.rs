fn main() {
    println!("{}", Solution::robot_with_string(String::from("zza")));
}

struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let bs = s.as_bytes();
        let mut stack = vec![];
        let mut result = vec![];
        let mut counter = vec![0; 26];
        for &c in bs {
            counter[(c - b'a') as usize] += 1;
        }
        let mut min_char = b'a';
        for &c in bs {
            counter[(c - b'a') as usize] -= 1;
            stack.push(c);
            while min_char < b'z' && counter[(min_char - b'a') as usize] == 0 {
                min_char += 1;
            }
            while !stack.is_empty() && stack[stack.len() - 1] <= min_char {
                result.push(stack.pop().unwrap());
            }
        }
        String::from_utf8(result).unwrap()
    }
}
