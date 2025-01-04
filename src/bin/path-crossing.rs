fn main() {
    println!("{}", Solution::is_path_crossing("NES".to_string()));
}

struct Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        const SHIFT: i32 = 10000;
        const LEN: i32 = 20001;
        let mut bit_map = new_bit_map((LEN * LEN) as usize);
        let mut x = 0;
        let mut y = 0;
        set_bit(((x + SHIFT) * LEN + (y + SHIFT)) as usize, &mut bit_map);
        for c in path.chars() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                'W' => x -= 1,
                _ => unreachable!(),
            }
            if get_bit(((x + SHIFT) * LEN + (y + SHIFT)) as usize, &mut bit_map) {
                return true;
            } else {
                set_bit(((x + SHIFT) * LEN + (y + SHIFT)) as usize, &mut bit_map);
            }
        }
        false
    }
}

#[inline]
fn new_bit_map(total: usize) -> Vec<u64> {
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

#[inline]
fn get_bit(n: usize, bit_map: &mut Vec<u64>) -> bool {
    let i = n >> 6;
    let k = n & 63;
    (bit_map[i] >> k) & 1 > 0
}
