fn main() {
    println!(
        "{}",
        Solution::repeat_limited_string("cczazcc".to_string(), 3)
    );
    println!(
        "{}",
        Solution::repeat_limited_string("aababab".to_string(), 2)
    );
}

struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let s = s.as_bytes();
        let rl = repeat_limit as usize;
        let mut freq: Vec<usize> = vec![0; 26];
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut freq_stack = vec![];
        for (i, &c) in freq.iter().enumerate() {
            if c > 0 {
                freq_stack.push(i);
            }
        }
        let mut ans: Vec<u8> = vec![];
        while !freq_stack.is_empty() {
            let idx = freq_stack.pop().unwrap();
            let max_c = idx as u8 + b'a';
            let mut rr = rl;
            while rr > 0 && freq[idx] > 0 {
                ans.push(max_c);
                rr -= 1;
                freq[idx] -= 1;
            }
            if freq[idx] == 0 {
                continue;
            }
            if rr == 0 {
                if !freq_stack.is_empty() {
                    let next_idx = *freq_stack.last().unwrap();
                    let next_max_c = next_idx as u8 + b'a';
                    ans.push(next_max_c);
                    freq[next_idx] -= 1;
                    if freq[next_idx] == 0 {
                        freq_stack.pop();
                    }
                    freq_stack.push(idx);
                } else {
                    break;
                }
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
