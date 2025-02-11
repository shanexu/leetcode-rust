fn main() {
    println!(
        "{}",
        Solution::remove_occurrences(
            "kpygkivtlqoockpygkivtlqoocssnextkqzjpycbylkaondsskpygkpygkivtlqoocssnextkqzjpkpygkivtlqoocssnextkqzjpycbylkaondsycbylkaondskivtlqoocssnextkqzjpycbylkaondssnextkqzjpycbylkaondshijzgaovndkjiiuwjtcpdpbkrfsi".to_string(),
            "kpygkivtlqoocssnextkqzjpycbylkaonds".to_string())
    );
    println!(
        "{}",
        Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string())
    );
    println!(
        "{}",
        Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let s = s.as_bytes();
        let part = part.as_bytes();
        let lps = construct_lps(part);
        let mut byte_stack: Vec<u8> = vec![];
        let mut state_stack = vec![0];
        for &b in s {
            let mut state = *state_stack.last().unwrap();
            loop {
                if part[state] == b {
                    let ns = state + 1;
                    if ns == part.len() {
                        byte_stack.truncate(byte_stack.len() - state);
                        state_stack.truncate(state_stack.len() - state);
                    } else {
                        byte_stack.push(b);
                        state_stack.push(ns);
                    }
                    break;
                } else {
                    if state != 0 {
                        state = lps[state - 1];
                    } else {
                        byte_stack.push(b);
                        state_stack.push(0);
                        break;
                    }
                }
            }
        }
        String::from_utf8(byte_stack).unwrap()
    }
}

#[inline]
fn construct_lps(pat: &[u8]) -> Vec<usize> {
    let mut len = 0;
    let mut lps = vec![0; pat.len()];
    let mut i = 1;
    while i < pat.len() {
        if pat[i] == pat[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }
    lps
}
