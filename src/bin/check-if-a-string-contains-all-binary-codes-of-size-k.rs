fn main() {
    println!("{}", Solution::has_all_codes("00110110".to_string(), 2));
    println!("{}", Solution::has_all_codes("0110".to_string(), 1));
    println!("{}", Solution::has_all_codes("0110".to_string(), 2));
}

struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let mut bit_map = new_bit_map(k);
        let s = s.as_bytes();
        let n = s.len();
        if n < k {
            return false;
        }
        let mut num = 0u32;
        let mask = (1 << k) - 1;
        for i in 0..k {
            num = num * 2 + (s[i] - b'0') as u32;
        }
        set_bit(num as usize, &mut bit_map);
        for i in 1..=(n - k) {
            num <<= 1;
            num += (s[i + k - 1] - b'0') as u32;
            num &= mask;
            set_bit(num as usize, &mut bit_map);
        }
        for &v in bit_map.iter() {
            if v != u64::MAX {
                return false;
            }
        }
        true
    }
}

#[inline]
fn new_bit_map(k: usize) -> Vec<u64> {
    let total = 1 << k;
    let len = (total + 63) >> 6;
    let mut bits = vec![0; len];
    let remain = total & 63;
    if remain != 0 {
        bits[len - 1] = !0 << remain;
    }
    bits
}

#[inline]
fn set_bit(n: usize, bit_map: &mut Vec<u64>) {
    let i = n >> 6;
    let k = n & 63;
    bit_map[i] |= 1 << k;
}