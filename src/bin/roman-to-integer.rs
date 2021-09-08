/*
1. I can be placed before V (5) and X (10) to make 4 and 9.
2. X can be placed before L (50) and C (100) to make 40 and 90.
3. C can be placed before D (500) and M (1000) to make 400 and 900.
*/
fn main() {
    println!("{}", Solution::roman_to_int(String::from("")));
    println!("{}", Solution::roman_to_int(String::from("III")));
    println!("{}", Solution::roman_to_int(String::from("IV")));
    println!("{}", Solution::roman_to_int(String::from("IX")));
    println!("{}", Solution::roman_to_int(String::from("LVIII")));
    println!("{}", Solution::roman_to_int(String::from("MCMXCIV")));
}

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        const I: u8 = b'I';
        const V: u8 = b'V';
        const X: u8 = b'X';
        const L: u8 = b'L';
        const C: u8 = b'C';
        const D: u8 = b'D';
        const M: u8 = b'M';
        let mut result: i32 = 0;
        let bytes = s.as_bytes();
        let mut i = 0;
        loop {
            if i >= bytes.len() {
                break;
            }
            let item = bytes[i];
            let has_next = i < bytes.len() - 1;
            if has_next {
                let next_item = bytes[i + 1];
                match (item, next_item) {
                    (I, V) => {
                        result += 4;
                        i += 2;
                        continue;
                    }
                    (I, X) => {
                        result += 9;
                        i += 2;
                        continue;
                    }
                    (X, L) => {
                        result += 40;
                        i += 2;
                        continue;
                    }
                    (X, C) => {
                        result += 90;
                        i += 2;
                        continue;
                    }
                    (C, D) => {
                        result += 400;
                        i += 2;
                        continue;
                    }
                    (C, M) => {
                        result += 900;
                        i += 2;
                        continue;
                    }
                    _ => {}
                }
            }
            match item {
                I => {
                    result += 1;
                }
                V => {
                    result += 5;
                }
                X => {
                    result += 10;
                }
                L => {
                    result += 50;
                }
                C => {
                    result += 100;
                }
                D => {
                    result += 500;
                }
                M => {
                    result += 1000;
                }
                _ => {}
            }
            i += 1;
        }
        result
    }
}
