fn main() {}

struct Solution;

impl Solution {
    pub fn minimize_xor(mut num1: i32, mut num2: i32) -> i32 {
        let mut count1 = bit_count(num1);
        let count2 = bit_count(num2);
        while count1 > count2 {
            num1 &= num1 - 1;
            count1 -= 1;
        }
        while count1 < count2 {
            num1 |= num1 + 1;
            count1 += 1
        }
        num1
    }
}

fn bit_count(mut var0: i32) -> i32 {
    var0 -= var0 >> 1 & 1431655765;
    var0 = (var0 & 858993459) + (var0 >> 2 & 858993459);
    var0 = (var0 + (var0 >> 4)) & 252645135;
    var0 += var0 >> 8;
    var0 += var0 >> 16;
    var0 & 63
}