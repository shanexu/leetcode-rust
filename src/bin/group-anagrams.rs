fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat")
        ])
    );
}

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], usize> = HashMap::new();
        let mut strs = strs;
        let mut result: Vec<Vec<String>> = vec![];
        while let Some(s) = strs.pop() {
            let flag = hash(&s);
            if let Some(i) = map.get_mut(&flag) {
                if let Some(v) = result.get_mut(*i) {
                    v.push(s);
                }
            } else {
                map.insert(flag, result.len());
                result.push(vec![s]);
            }
        }
        result
    }
}

#[inline]
fn hash(s: &String) -> [u8; 26] {
    let mut array: [u8; 26] = [0; 26];
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        let i = (b - b'a') as usize;
        array[i] = array[i] + 1;
    }
    array
}
