use leetcode_rust::vec_string;

fn main() {
    println!(
        "{}",
        Solution::eval_rpn(vec_string!["2", "1", "+", "3", "*"])
    );
    println!(
        "{}",
        Solution::eval_rpn(vec_string!["4", "13", "5", "/", "+"])
    );
    println!(
        "{}",
        Solution::eval_rpn(vec_string![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
        ])
    );
}

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let op2 = stack.pop().unwrap();
                    let op1 = stack.pop().unwrap();
                    stack.push(op1 + op2);
                }
                "-" => {
                    let op2 = stack.pop().unwrap();
                    let op1 = stack.pop().unwrap();
                    stack.push(op1 - op2);
                }
                "*" => {
                    let op2 = stack.pop().unwrap();
                    let op1 = stack.pop().unwrap();
                    stack.push(op1 * op2);
                }
                "/" => {
                    let op2 = stack.pop().unwrap();
                    let op1 = stack.pop().unwrap();
                    stack.push(op1 / op2);
                }
                _ => {
                    stack.push(token.parse::<i32>().unwrap());
                }
            }
        }
        stack[0]
    }
}
