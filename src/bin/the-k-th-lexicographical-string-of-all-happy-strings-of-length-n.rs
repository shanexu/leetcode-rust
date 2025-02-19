fn main() {
    assert_eq!("c".to_string(), Solution::get_happy_string(1, 3));
    assert_eq!("".to_string(), Solution::get_happy_string(1, 4));
    assert_eq!("cab".to_string(), Solution::get_happy_string(3, 9));
    assert_eq!("ba".to_string(), Solution::get_happy_string(2, 3));
}

struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut k = k as usize;
        let mut path: Vec<u8> = vec![0; n];
        if help(0, n, &mut k, &mut path) {
            String::from_utf8(path).unwrap()
        } else {
            "".to_string()
        }
    }
}

fn help(idx: usize, n: usize, k: &mut usize, path: &mut Vec<u8>) -> bool {
    if idx == n {
        *k -= 1;
        if *k == 0 {
            return true;
        }
        return false;
    }
    let prev = if idx == 0 { 0 } else { path[idx - 1] };
    for b in b'a'..=b'c' {
        if b == prev {
            continue;
        }
        path[idx] = b;
        if help(idx + 1, n, k, path) {
            return true;
        }
        path[idx] = 0;
    }
    false
}