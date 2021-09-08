fn main() {
    println!("{:?}", Solution::longest_common_prefix(vec![String::from("flower"), String::from("flow"), String::from("flight")]));
    println!("{:?}", Solution::longest_common_prefix(vec![String::from("dog"), String::from("racecar"), String::from("car")]))
}

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }
        if strs.len() == 1 {
            return strs[0].clone();
        }
        let mut prefix_vec = Vec::new();
        let head = &strs[0];
        let tail = &strs[1..];
        for i in 0..head.len() {
            let c = head.chars().nth(i).unwrap();
            if tail.into_iter().all(|s| s.chars().nth(i).filter(|&x| c == x).is_some()) {
                prefix_vec.push(c as u8);
            } else {
                break;
            }
        }
        String::from_utf8(prefix_vec).unwrap()
    }
}
