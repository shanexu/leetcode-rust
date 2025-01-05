fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let s = s.as_bytes();
        String::from_utf8(
            s.chunks(2 * k)
                .map(|ch| {
                    let mut ch = ch.to_vec();
                    if ch.len() < k {
                        ch.reverse();
                    } else {
                        &mut ch[0..k].reverse();
                    }
                    ch
                })
                .flatten()
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }
}
