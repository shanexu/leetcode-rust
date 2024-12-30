fn main() {
    println!("{:?}", Solution::plus_one(vec![1, 2, 3]));
    println!("{:?}", Solution::plus_one(vec![9]));
    println!("{:?}", Solution::plus_one(vec![9, 9]));
}

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        for i in (0..digits.len()).rev() {
            let mut d = digits[i];
            d = d + carry;
            if d == 10 {
                d = 0;
                carry = 1;
            } else {
                carry = 0;
            }
            digits[i] = d;
            if carry == 0 {
                break;
            }
        }
        if carry > 0 {
            digits.insert(0, carry);
        }
        digits
    }
}
