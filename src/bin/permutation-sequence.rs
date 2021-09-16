fn main() {
    for i in 0..24 {
        println!("{}", Solution::get_permutation(4, i+1));
    }
}

struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut result = Vec::with_capacity(n);
        let mut factorials: Vec<usize> = vec![1; n];
        for i in 0..n {
            result.push(b'1' + i as u8);
            if i != 0 {
                factorials[n - i - 1] = factorials[n - i] * i;
            }
        }
        let mut k = k as usize - 1;
        for i in 0..n {
            let f = factorials[i];
            let t = k / f;
            k = k - t * f;
            let target = result[i + t];
            for j in (i..(i+t)).rev() {
                result[j+1] = result[j];
            }
            result[i] = target;
        }
        String::from_utf8(result).unwrap()
    }
}
