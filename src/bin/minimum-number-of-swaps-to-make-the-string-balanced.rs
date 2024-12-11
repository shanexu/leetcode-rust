fn main() {
    println!("{}", Solution::min_swaps(String::from("]]][[[")));
}

struct Solution;

impl Solution {
    pub fn min_swaps(mut s: String) -> i32 {
        let bytes = unsafe { s.as_bytes_mut() };
        let len = bytes.len();
        let half_len = len / 2;
        let mut swap_count = 0;
        let mut open_count = 0;
        let mut close_clount = 0;
        let mut ridx = len;
        for i in 0..len {
            let c = bytes[i];
            if c == b'[' {
                open_count += 1;
            } else if c == b']' {
                close_clount += 1;
            }
            if close_clount > open_count {
                loop {
                    ridx -= 1;
                    if bytes[ridx] == b']' {
                        bytes.swap(i, ridx);
                        open_count += 1;
                        close_clount -= 1;
                        swap_count += 1;
                        break;
                    }
                }
            }
            if open_count >= half_len {
                break;
            }
        }
        swap_count
    }
}
