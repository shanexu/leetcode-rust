fn main() {
    println!(
        "{:?}",
        Solution::diff_ways_to_compute("2*3-4*5".to_string())
    );
}

struct Solution;

use std::collections::HashMap;
use crate::Exp::{Operand, Operator};
use crate::Op::{Add, Minus, Multiply};
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        fn helper(
            exps: &[Exp],
            pair@(i, j): (usize, usize),
            memo: &mut HashMap<(usize, usize), Vec<i32>>,
        ) -> Vec<i32> {
            if i + 1 == j {
                let exp = exps[i];
                return match exp {
                    Operand(number) => vec![number],
                    _ => vec![]
                };
            }
            if let Some(res) = memo.get(&pair) {
                return res.clone();
            }
            let mut res = vec![];
            for k in i..j {
                let exp = exps[k];
                match exp {
                    Operator(op) => {
                        let res_pre = helper(exps, (i, k), memo);
                        let res_post = helper(exps, (k+1, j), memo);
                        for p in 0..res_pre.len() {
                            for q in 0..res_post.len() {
                                res.push(op.apply(res_pre[p], res_post[q])) ;
                            }
                        }
                    }
                    _ => {}
                }
            }
            memo.insert(pair, res.clone());
            res
        }

        let expression = expression.as_bytes();
        let exps = parse_expression(expression);
        helper(&exps, (0, exps.len()), &mut HashMap::new())
    }
}

fn parse_expression(expression: &[u8]) -> Vec<Exp> {
    let mut res = vec![];
    let mut j = 0;
    for i in 0..expression.len() {
        let b = expression[i];
        if !b.is_ascii_digit() {
            let mut num = 0;
            for k in j..i {
                num *= 10;
                num += (expression[k] - b'0') as i32;
            }
            res.push(Operand(num));
            if b == b'+' {
                res.push(Operator(Add));
            } else if b == b'-' {
                res.push(Operator(Minus));
            } else if b == b'*' {
                res.push(Operator(Multiply));
            }
            j = i + 1;
        }
    }
    let mut num = 0;
    for k in j..expression.len() {
        num *= 10;
        num += (expression[k] - b'0') as i32;
    }
    res.push(Operand(num));
    res
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Op {
    Add,
    Minus,
    Multiply,
}

impl Op {
    fn apply(&self, left: i32, right: i32) -> i32 {
        match self {
            Op::Add => left + right,
            Op::Minus => left - right,
            Op::Multiply => left * right,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Exp {
    Operand(i32),
    Operator(Op),
}
