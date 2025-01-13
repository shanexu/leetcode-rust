fn main() {
    println!(
        "{:?}",
        Solution::restore_ip_addresses("25525511135".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn backtrace<'a>(
            index: usize,
            n: usize,
            s: &'a [u8],
            curr: &mut Vec<&'a [u8]>,
            ans: &mut Vec<String>,
        ) {
            if index >= n && curr.len() == 4 {
                let mut ip = Vec::with_capacity(15);
                for i in 0..4 {
                    ip.extend_from_slice(curr[i]);
                    if i < 3 {
                        ip.push(b'.');
                    }
                }
                ans.push(String::from_utf8(ip).unwrap());
                return;
            }
            if index >= n || curr.len() >= 4 {
                return;
            }
            let mut segment = 0i32;
            for j in index..(index + 3).min(n) {
                segment = segment * 10 + (s[j] - b'0') as i32;
                if segment > 255 || (s[index] == b'0' && index != j) {
                    break;
                }
                curr.push(&s[index..(j + 1)]);
                backtrace(j + 1, n, s, curr, ans);
                curr.pop();
            }
        }
        let mut ans = vec![];
        let s = s.as_bytes();
        backtrace(0, s.len(), s, &mut vec![], &mut ans);
        ans
    }
}
