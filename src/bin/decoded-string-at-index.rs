fn main() {
    println!(
        "{}",
        Solution::decode_at_index("y75lgfqyn4re8treuyrs9pqxfgvf3rrtqxr6lrm8ymxawwf97jcm5itz8dpvjig3o9iartdajjeoo58ipu6wmuozzpjzzfzrciy6".to_string(), 292404628)
    );
    println!(
        "{}",
        Solution::decode_at_index("xo78xylzu6".to_string(), 342)
    );
}

#[allow(dead_code)]
fn decode(s: String) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    for &b in s.as_bytes() {
        if b.is_ascii_digit() {
            let t = (b - b'0' - 1) as usize;
            let cloned = res.clone();
            for _ in 0..t {
                res.extend(cloned.clone());
            }
        } else {
            res.push(b);
        }
    }
    res
}
struct Solution;

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        char::from(decode_at(s, k)).to_string()
    }
}

#[inline(always)]
fn split(s: &[u8]) -> Vec<(&[u8], usize, i64)> {
    let mut res: Vec<(&[u8], usize, i64)> = vec![];
    let mut i = 0;
    let mut acc = 0;
    for (j, &c) in s.iter().enumerate() {
        if c.is_ascii_digit() {
            let repeat = (c - b'0') as usize;
            let slice = &s[i..j];
            if slice.len() == 0 {
                let prev_idx = res.len() - 1;
                let (prev_slice, prev_repeat, prev_acc) = res[prev_idx];
                acc = prev_acc;
                res[prev_idx] = (prev_slice, prev_repeat * repeat, acc);
                acc = (acc + prev_slice.len() as i64) * prev_repeat as i64 * repeat as i64;
            } else {
                res.push((slice, repeat, acc));
                acc = (acc + slice.len() as i64) * repeat as i64;
            }
            i = j + 1;
        }
    }
    if !s[s.len() - 1].is_ascii_digit() {
        res.push((&s[i..], 1, acc));
    }
    res
}

#[inline(always)]
fn decode_at(s: String, k: i32) -> u8 {
    let mut k = k as i64;
    let s = s.as_bytes();
    let parts = split(s);
    for &(ss, _, pl) in parts.iter().rev() {
        k = k % (ss.len() as i64 + pl);
        if k == 0 {
            return ss[ss.len() - 1];
        }
        if k <= pl {
            continue;
        }
        k -= pl;
        return ss[(k - 1) as usize];
    }
    0
}
