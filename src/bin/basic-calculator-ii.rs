fn main() {
    println!("{}", Solution::calculate("12 * 34 - 56 *  78".to_string()));
}

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut num = 0;
        let mut operators: Vec<u8> = vec![];
        let mut operands: Vec<i32> = vec![];
        for &b in s {
            if b.is_ascii_digit() {
                num = num * 10 + (b - b'0') as i32;
            } else if is_operator(b) {
                operands.push(num);
                num = 0;
                while !operators.is_empty()
                    && precedence(operators[operators.len() - 1]) >= precedence(b)
                {
                    let op = operators.pop().unwrap();
                    let n2 = operands.pop().unwrap();
                    let n1 = operands.pop().unwrap();
                    operands.push(apply_op(op, n1, n2));
                }
                operators.push(b);
            }
        }
        operands.push(num);
        while let Some(op) = operators.pop() {
            let n2 = operands.pop().unwrap();
            let n1 = operands.pop().unwrap();
            operands.push(apply_op(op, n1, n2));
        }
        operands[0]
    }
}

#[inline]
fn apply_op(op: u8, n1: i32, n2: i32) -> i32 {
    match op {
        b'+' => n1 + n2,
        b'-' => n1 - n2,
        b'*' => n1 * n2,
        b'/' => n1 / n2,
        _ => unreachable!(),
    }
}

#[inline]
fn is_operator(c: u8) -> bool {
    c == b'+' || c == b'-' || c == b'*' || c == b'/'
}

#[inline]
fn precedence(op: u8) -> i32 {
    match op {
        b'+' | b'-' => 1,
        b'*' | b'/' => 2,
        _ => unreachable!(),
    }
}
