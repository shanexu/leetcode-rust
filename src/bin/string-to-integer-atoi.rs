fn main() {
    println!("{}", Solution::my_atoi(String::from("  -42 as")));
    println!("{}", Solution::my_atoi(String::from(" 100")));
    println!("{}", Solution::my_atoi(String::from("")));
    println!("{}", Solution::my_atoi(String::from("0012")));
    println!("{}", Solution::my_atoi(String::from("+2147483647")));
    println!("{}", Solution::my_atoi(String::from("+21474836480")));
    println!("{}", Solution::my_atoi(String::from("-2147483648")));
    println!("{}", Solution::my_atoi(String::from("-2147483649")));
}

struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {

        enum State {
            Whitepace,
            Sign,
            Digits,
        }

        let mut state = State::Whitepace;
        let mut sign: i32 = 0;
        let mut n: i32 = 0;
        for b in s.chars() {
            match state {
                State::Whitepace => {
                    if b == ' ' {
                        continue;
                    }
                    if b == '+' {
                        state = State::Sign;
                        sign = 1;
                    } else if b == '-' {
                        state = State::Sign;
                        sign = -1;
                    } else if b >= '0' && b <= '9' {
                        state = State::Digits;
                        sign = 1;
                        n = n * 10 + (sign * (b as i32 - '0' as i32));
                    } else {
                        return 0;
                    }
                }
                State::Sign => {
                    if b >= '0' && b <= '9' {
                        state = State::Digits;
                        n = n * 10 + (sign * (b as i32 - '0' as i32));
                    } else {
                        return 0;
                    }
                }
                State::Digits => {
                    if b >= '0' && b <= '9' {
                        match n
                            .checked_mul(10)
                            .and_then(|v| v.checked_add(sign * (b as i32 - '0' as i32)))
                        {
                            Some(v) => {
                                n = v;
                            }
                            None => {
                                if sign > 0 {
                                    return i32::MAX;
                                } else {
                                    return i32::MIN;
                                }
                            }
                        }
                    } else {
                        return n;
                    }
                }
            }
        }
        n
    }
}
