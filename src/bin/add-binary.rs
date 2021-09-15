fn main() {
    println!(
        "{}",
        Solution::add_binary(String::from("11"), String::from("1"))
    );
    println!(
        "{}",
        Solution::add_binary(String::from("10"), String::from("1"))
    );
}

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut max_len = a.len();
        if max_len < b.len() {
            max_len = b.len();
        }
        let a = a.as_bytes();
        let mut a_i = a.len() as i32 - 1;
        let b = b.as_bytes();
        let mut b_i = b.len() as i32 - 1;
        let mut vs = vec![b'0'; max_len + 1];
        let mut carry = 0;
        for i in (0..(max_len + 1)).rev() {
            let x = if a_i >= 0 { a[a_i as usize] - b'0' } else { 0 };
            let y = if b_i >= 0 { b[b_i as usize] - b'0' } else { 0 };
            a_i -= 1;
            b_i -= 1;
            let v = x + y + carry;
            vs[i] = b'0' + (v % 2);
            carry = v / 2;
        }
        if vs[0] == b'0' {
            let (_, t) = vs.split_at(1);
            String::from_utf8(t.to_vec()).unwrap()
        } else {
            String::from_utf8(vs).unwrap()
        }
    }
}
