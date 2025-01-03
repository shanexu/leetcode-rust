fn main() {
    println!("{}", Solution::calculate("- (3 + (4 + 5))".to_string()));
    // println!("{}", Solution::calculate("11+22-(1+2)".to_string()));
}

struct Solution;

use Exp::{Open, Operand, Operator};
use Op::{Add, Minus};
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut digits: Vec<u8> = vec![];
        let mut stack = vec![Operand(0)];
        for &b in s {
            if b.is_ascii_digit() {
                digits.push(b);
            } else if b == b'+' {
                if !digits.is_empty() {
                    stack.push(Operand(digits_to_number(&digits)));
                    digits.clear();
                }
                eval_stack(&mut stack);
                stack.push(Operator(Add));
            } else if b == b'-' {
                if !digits.is_empty() {
                    stack.push(Operand(digits_to_number(&digits)));
                    digits.clear();
                }
                eval_stack(&mut stack);
                stack.push(Operator(Minus));
            } else if b == b'(' {
                stack.push(Open);
            } else if b == b')' {
                if !digits.is_empty() {
                    stack.push(Operand(digits_to_number(&digits)));
                    digits.clear();
                }
                eval_stack(&mut stack);
                popup_open(&mut stack);
            }
        }
        if !digits.is_empty() {
            stack.push(Operand(digits_to_number(&digits)));
            digits.clear();
        }
        eval_stack(&mut stack);
        if let Some(Operand(ans)) = stack.pop() {
            ans
        } else {
            0
        }
    }
}

fn eval_stack(stack: &mut Vec<Exp>) {
    while stack.len() >= 3 {
        let x = stack.pop();
        let y = stack.pop();
        let z = stack.pop();
        match (z, y, x) {
            (Some(Operand(n1)), Some(Operator(op)), Some(Operand(n2))) => {
                stack.push(Operand(op.apply(n1, n2)));
            }
            (Some(Open), Some(Operator(Minus)), Some(Operand(n1))) => {
                stack.push(Open);
                stack.push(Operand(0 - n1));
            }
            (Some(z), Some(y), Some(x)) => {
                stack.push(z);
                stack.push(y);
                stack.push(x);
                return;
            }
            _ => {}
        }
    }
}
fn popup_open(stack: &mut Vec<Exp>) {
    let x = stack.pop();
    let y = stack.pop();
    match (y, x) {
        (Some(Open), Some(n @ Operand(_))) => {
            stack.push(n);
        }
        (Some(y), Some(x)) => {
            stack.push(y);
            stack.push(x);
        }
        _ => {}
    }
}

#[inline]
fn digits_to_number(arr: &Vec<u8>) -> i32 {
    let mut num = 0;
    for &b in arr {
        num = num * 10 + (b - b'0') as i32;
    }
    num
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Add,
    Minus,
}

impl Op {
    fn apply(&self, n1: i32, n2: i32) -> i32 {
        match self {
            Add => n1 + n2,
            Minus => n1 - n2,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Exp {
    Operator(Op),
    Operand(i32),
    Open,
}
