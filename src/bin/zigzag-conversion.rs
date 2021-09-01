fn main() {
    let cases = [("PAYPALISHIRING", 3), ("ABCDEFG", 2)];
    for (s, n) in cases.iter() {
        println!("{}", Solution::convert(String::from(*s), *n as i32));
    }
}

struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let bs = s.as_bytes();
        let l = bs.len();
        let mut ns = String::with_capacity(l);
        if num_rows == 2 {
            for i in (0..l).step_by(2) {
                ns.push(bs[i] as char);
            }
            for i in (1..l).step_by(2) {
                ns.push(bs[i] as char);
            }
            return ns;
        }
        let r = num_rows as usize;
        let t = (r + r - 2) as usize;
        let c = (l / t + (if l % t == 0 { 0 } else { 1 })) * 2;
        for i in 0..r {
            for j in 0..c {
                let n = j / 2;
                let m;
                if j % 2 == 1 {
                    if i == 0 || i == r - 1 {
                        continue;
                    }
                    let k = r + (r - 2 - i);
                    m = n * t + k;
                } else {
                    let k = i;
                    m = n * t + k;
                }
                if m < l {
                    ns.push(bs[m] as char);
                }
            }
        }
        ns
    }
}
