fn main() {
    println!("{}", Solution::count_and_say(6))
}

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        cas(n)
    }
}

fn cas(n: i32) -> String {
    let mut i = 2;
    let mut bs = vec![b'1'];
    while i <= n {
        let mut cs = vec![];
        let mut p = b'\0';
        let mut c = 0;
        for &b in bs.iter() {
            if b == p {
                c += 1;
            } else {
                if c > 0 {
                    cs.push(b'0' + c as u8);
                    cs.push(p);
                }
                p = b;
                c = 1;
            }
        }
        if c > 0 {
            cs.push(b'0' + c as u8);
            cs.push(p);
        }
        bs = cs;
        i += 1;
    }
    String::from_utf8(bs).unwrap()
}
