fn main() {
    println!("{}", Solution::digit_sum("1234".to_string(), 3));
}

struct Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let k = k as usize;
        let mut bs = s.as_bytes().to_vec();
        let mut tmp: Vec<u8> = vec![];

        while bs.len() > k {
            for chunk in bs.chunks(k) {
                let mut s = chunk.iter().map(|d| (d - b'0') as i32).sum::<i32>();
                let mut count = 0;
                loop {
                    count += 1;
                    tmp.push((s % 10) as u8 + b'0');
                    s /= 10;
                    if s == 0 {
                        break;
                    }
                }
                let start_idx = tmp.len() - count;
                let slice = &mut tmp[start_idx..];
                slice.reverse();
            }
            bs.clear();
            std::mem::swap(&mut bs, &mut tmp);
        }

        String::from_utf8(bs).unwrap()
    }
}
