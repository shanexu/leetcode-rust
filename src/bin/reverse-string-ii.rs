fn main() {
    println!("{}", Solution::reverse_str("abc".to_string(), 6))
}

struct Solution;

impl Solution {
    pub fn reverse_str(mut s: String, k: i32) -> String {
        let k = k as usize;
        let bs = unsafe { s.as_bytes_mut() };
        let n = bs.len();
        for i in (0..n).step_by(2 * k) {
            let end = (i + k).min(n); // 确定处理的结束位置
            bs[i..end].reverse(); // 反转前 k 个字节
        }
        s
    }
}
