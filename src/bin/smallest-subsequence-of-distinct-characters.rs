fn main() {
    assert_eq!(
        Solution::smallest_subsequence("cdadabcc".to_string()),
        "adbc".to_string()
    );
    assert_eq!(
        Solution::smallest_subsequence("abcd".to_string()),
        "abcd".to_string()
    );
    assert_eq!(
        Solution::smallest_subsequence("ecbacba".to_string()),
        "eacb".to_string()
    );
    assert_eq!(
        Solution::smallest_subsequence("leetcode".to_string()),
        "letcod".to_string()
    );
    assert_eq!(
        Solution::smallest_subsequence("bcabc".to_string()),
        "abc".to_string()
    );
    assert_eq!(
        Solution::smallest_subsequence("cbacdcbc".to_string()),
        "acdb".to_string()
    );
}

struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let bs = s.as_bytes();
        let mut last_index: Vec<i32> = vec![-1; 26];
        bs.iter().enumerate().for_each(|(i, &b)| {
            last_index[(b - b'a') as usize] = i as i32;
        });
        let mut bit_map: u32 = 0;
        let mut stack: Vec<u8> = vec![];
        for (i, &b) in bs.iter().enumerate() {
            if (bit_map >> (b - b'a')) & 1 == 1 {
                continue;
            }
            while !stack.is_empty()
                && stack[stack.len() - 1] > b
                && last_index[(stack[stack.len() - 1] - b'a') as usize] > i as i32
            {
                let a = stack.pop().unwrap();
                bit_map ^= (1 << (a - b'a')) as u32;
            }
            stack.push(b);
            bit_map |= (1 << (b - b'a')) as u32;
        }
        String::from_utf8(stack).unwrap()
    }
}
