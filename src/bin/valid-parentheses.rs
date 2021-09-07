fn main() {
    println!("{}", Solution::is_valid(String::from("()")));
    println!("{}", Solution::is_valid(String::from("()[]{}")));
    println!("{}", Solution::is_valid(String::from("(]")));
    println!("{}", Solution::is_valid(String::from("([)]")));
    println!("{}", Solution::is_valid(String::from("{[]}")));
}

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        const PL: u8 = '(' as u8;
        const PR: u8 = ')' as u8;
        const SL: u8 = '[' as u8;
        const SR: u8 = ']' as u8;
        const CL: u8 = '{' as u8;
        const CR: u8 = '}' as u8;
        let mut stack: Vec<u8> = Vec::new();
        for b in s.as_bytes() {
            let b = *b;
            match stack.last() {
                None => stack.push(b),
                Some(v) => {
                    let v = *v;
                    if b == PR {
                        if v == PL {
                            stack.pop();
                        } else {
                            return false;
                        }
                    } else if b == SR {
                        if v == SL {
                            stack.pop();
                        } else {
                            return false;
                        }
                    } else if b == CR {
                        if v == CL {
                            stack.pop();
                        } else {
                            return false;
                        }
                    } else {
                        stack.push(b);
                    }
                }
            }
        }
        stack.is_empty()
    }
}
