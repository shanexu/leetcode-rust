fn main() {
    println!("{}", Solution::robot_with_string(String::from("bydizfve")));
}

struct Solution;

use std::collections::BTreeSet;
impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let bs = s.as_bytes();
        let n = bs.len();
        let mut tree = BTreeSet::new();
        for i in 0..bs.len() {
            tree.insert((bs[i], i));
        }
        let mut i = 0;
        let mut stack: Vec<u8> = vec![];
        let mut result: Vec<u8> = vec![];
        while result.len() < n {
            if tree.is_empty() {
                while let Some(c) = stack.pop() {
                    result.push(c);
                }
                continue;
            }
            if !stack.is_empty() && stack[stack.len() - 1] <= tree.first().unwrap().0 {
                let c = stack.pop().unwrap();
                result.push(c);
                continue;
            }
            let (c, j) = tree.pop_first().unwrap();
            result.push(c);
            for k in i..j {
                let v = bs[k];
                stack.push(v);
                tree.remove(&(v, k));
            }
            i = j + 1;
        }
        String::from_utf8(result).unwrap()
    }
}
