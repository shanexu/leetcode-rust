fn main() {
    // let num1: &[u8] = &[b'9', b'9', b'9'];
    // let num2: &[u8] = &[b'9', b'9', b'9'];
    // let mut out = vec![b'0'; num1.len() + num2.len()];
    // multiply(num1, num2, &mut out);
    // println!("{}", String::from_utf8(out).unwrap());
    println!(
        "{}",
        Solution::multiply(String::from("1234"), String::from("5678"))
    );
}

struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let mut out = vec![b'0'; num1.len() + num2.len()];
        multiply(num1, num2, &mut out);
        let pos = out.iter().position(|&x| x != b'0').unwrap_or(out.len());
        let bs = &out[pos..];
        if bs.len() == 0 {
            return String::from("0");
        }
        String::from_utf8_lossy(bs).to_string()
    }
}

pub fn add_slice(num: &[u8], out: &mut Vec<u8>, idx: usize) {
    let mut prompt: u8 = 0;
    for i in (idx..idx + num.len()).rev() {
        let value = num[i - idx] - b'0' + out[i] - b'0' + prompt;
        prompt = value / 10;
        out[i] = value % 10 + b'0';
    }
    for i in (0..idx).rev() {
        let value = out[i] - b'0' + prompt;
        prompt = value / 10;
        out[i] = value % 10 + b'0';
    }
}

pub fn multiply_digit(num: &[u8], digit: u8, out: &mut Vec<u8>, idx: usize) {
    let value_slice: &mut [u8] = &mut [0; 2];
    let digit = digit - b'0';
    for i in 0..num.len() {
        let value = (num[i] - b'0') * digit;
        value_slice[1] = value % 10 + b'0';
        value_slice[0] = value / 10 + b'0';
        add_slice(&value_slice, out, i + idx)
    }
}

pub fn multiply(op1: &[u8], op2: &[u8], out: &mut Vec<u8>) {
    for i in 0..op2.len() {
        multiply_digit(op1, op2[i], out, i)
    }
}
