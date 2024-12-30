fn main() {
    println!(
        "{}",
        Solution::repeat_limited_string("cczazcc".to_string(), 3)
    );
    println!("{}", Solution::repeat_limited_string("aababab".to_string(), 2));
}

struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let s = s.as_bytes();
        let rl = repeat_limit as usize;
        let mut freq: Vec<usize> = vec![0; 26];
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut tree = std::collections::BTreeSet::new();
        for (i, &c) in freq.iter().enumerate() {
            if c > 0 {
                let t = Triple::new(b'a' + i as u8, c, rl);
                tree.insert(t);
            }
        }
        let mut ans: Vec<u8> = vec![];
        let mut prev: Option<Triple> = None;
        while !tree.is_empty() && tree.last().unwrap().remain_repeat != 0 {
            let mut last = tree.pop_last().unwrap();
            ans.push(last.letter);
            last.remain_repeat -= 1;
            last.count -= 1;
            if let Some(mut p) = prev {
                if last.letter != p.letter {
                    tree.remove(&p);
                    p.remain_repeat = rl;
                    tree.insert(p);
                }
            }
            if last.count > 0 {
                prev = Some(last);
                tree.insert(last);
            } else {
                prev = None;
            }
        }
        String::from_utf8(ans).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Triple {
    letter: u8,
    count: usize,
    remain_repeat: usize,
}

impl Triple {
    fn new(letter: u8, count: usize, remain_repeat: usize) -> Self {
        Triple {
            letter,
            count,
            remain_repeat,
        }
    }
}

impl PartialOrd<Self> for Triple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.remain_repeat == 0 && other.remain_repeat != 0 {
            return Some(Ordering::Less);
        }
        if self.remain_repeat != 0 && other.remain_repeat == 0 {
            return Some(Ordering::Greater);
        }
        Some(self.letter.cmp(&other.letter))
    }
}

impl Ord for Triple {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.remain_repeat == 0 && other.remain_repeat != 0 {
            return Ordering::Less;
        }
        if self.remain_repeat != 0 && other.remain_repeat == 0 {
            return Ordering::Greater;
        }
        self.letter.cmp(&other.letter)
    }
}
