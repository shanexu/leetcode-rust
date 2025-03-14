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

fn get_next_state(pat: &[u8], state: usize, x: u8, pi: &[usize]) -> usize {
    if state < pat.len() && x == pat[state] {
        return state + 1;
    }

    // 使用前缀函数来避免重复计算
    let mut next_state = state;
    while next_state > 0 && pat[next_state] != x {
        next_state = pi[next_state - 1];
    }
    if pat[next_state] == x {
        next_state += 1;
    }
    next_state
}

fn compute_tf(pat: &[u8]) -> Vec<Vec<usize>> {
    let mut tf: Vec<Vec<usize>> = vec![vec![0; 26]; pat.len()]; // 只为小写字母分配内存
    let pi = construct_lps(pat); // 计算前缀函数

    for state in 0..pat.len() {
        for x in 0..26 {
            // 只考虑小写字母
            tf[state][x] = get_next_state(pat, state, (x as u8) + b'a', &pi);
        }
    }
    tf
}
