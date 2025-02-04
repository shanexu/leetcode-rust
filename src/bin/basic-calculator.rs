fn main() {
    // println!("{}", Solution::calculate("- (3 + (4 + 5))".to_string()));
    // println!("{}", Solution::calculate("11+22-(1+2)".to_string()));
    println!("{}", Solution::calculate("2-1 + 2".to_string()));
}

struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut num_buf: Vec<u8> = vec![];
        let mut is_last_char_open = true;
        let mut operators: Vec<u8> = vec![];
        let mut operands: Vec<i32> = vec![];
        for &(mut b) in s {
            if b.is_ascii_whitespace() {
                continue;
            } else if b.is_ascii_digit() {
                num_buf.push(b);
                is_last_char_open = false;
            } else {
                if !num_buf.is_empty() {
                    operands.push(digits_to_num(&num_buf));
                    num_buf.clear();
                }

                if b == b'(' {
                    operators.push(b);
                    is_last_char_open = true;
                } else if b == b')' {
                    is_last_char_open = false;
                    while let Some(top) = operators.pop() {
                        if top == b'(' {
                            break;
                        } else {
                            apply_op(top, &mut operands)
                        }
                    }
                } else {
                    if is_last_char_open && b == b'-' {
                        b = b'~';
                    }
                    is_last_char_open = false;
                    while let Some(&top) = operators.last() {
                        if top != b'(' && precedence(top) >= precedence(b) {
                            apply_op(operators.pop().unwrap(), &mut operands);
                        } else {
                            break;
                        }
                    }
                    operators.push(b);
                }
            }
        }
        if !num_buf.is_empty() {
            operands.push(digits_to_num(&num_buf));
        }
        while let Some(op) = operators.pop() {
            apply_op(op, &mut operands);
        }
        operands[0]
    }
}

#[inline]
fn precedence(op: u8) -> i32 {
    match op {
        b'+' | b'-' | b'~' => 1,
        b'*' | b'/' => 2,
        _ => 0,
    }
}

#[inline]
fn apply_op(op: u8, nums: &mut Vec<i32>) {
    let res = if op == b'~' {
        0 - nums.pop().unwrap()
    } else {
        let n2 = nums.pop().unwrap();
        let n1 = nums.pop().unwrap();
        match op {
            b'+' => n1 + n2,
            b'-' => n1 - n2,
            b'*' => n1 * n2,
            b'/' => n1 / n2,
            _ => unreachable!(),
        }
    };
    nums.push(res);
}

#[inline]
fn digits_to_num(digits: &[u8]) -> i32 {
    let mut num = 0;
    for &d in digits {
        num = num * 10 + (d - b'0') as i32;
    }
    num
}
