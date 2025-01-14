fn main() {
    let mut x = 1;
    let mut vec = vec![];
    while x <= 1000000000 {
        // println!("{}", x);
        vec.push(freq(x));
        x *= 2;
    }
    vec.sort();
    println!("{:?}", vec);
}

struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let f = freq(n);
        FREQ.binary_search(&f).is_ok()
    }
}

#[inline(always)]
fn freq(mut n: i32) -> i32 {
    let mut x = 0;
    while n > 0 {
        x += 10_i32.pow((n % 10) as u32);
        n /= 10;
    }
    x
}

const FREQ: &[i32] = &[
    10, 100, 1100, 10000, 10111, 100110, 223100, 1000010, 1010000, 1020210, 1100100, 2201000,
    10001121, 32000120, 100000000, 100000110, 100010101, 101011010, 102221100, 111001100,
    111110011, 120011220, 200110200, 212010011, 401001001, 1000031011, 1001010001, 1010100211,
    1100000110, 1111101111,
];
