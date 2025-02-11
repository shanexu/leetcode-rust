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
    print!(
        "{}",
        Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let s = s.as_bytes();
        let part = part.as_bytes();
        let part_len = part.len();
        let mut byte_stack: Vec<u8> = vec![];
        let mut state_stack: Vec<usize> = vec![0];
        let tf = compute_tf(part);
        for &b in s {
            let state = *state_stack.last().unwrap();
            let ns = tf[state][(b - b'a') as usize];
            if ns == part_len {
                byte_stack.truncate(byte_stack.len() - state);
                state_stack.truncate(state_stack.len() - state);
            } else {
                byte_stack.push(b);
                state_stack.push(ns);
            }
        }
        String::from_utf8(byte_stack).unwrap()
    }
}

fn get_next_state(pat: &[u8], state: usize, x: u8, tf: &Vec<Vec<usize>>) -> usize {
    if state < pat.len() && x == pat[state] {
        return state + 1;
    }
    if state == 0 {
        return 0;
    }
    tf[state - 1][(x - b'a') as usize]
}

fn compute_tf(pat: &[u8]) -> Vec<Vec<usize>> {
    let mut tf: Vec<Vec<usize>> = vec![vec![0; 26]; pat.len()];
    for state in 0..pat.len() {
        for x in 0..26 {
            tf[state][x] = get_next_state(pat, state, x as u8 + b'a', &mut tf);
        }
    }
    tf
}
