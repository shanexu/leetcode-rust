fn main() {
    // println!("{}", Solution::divide(10, 3));
    // println!("{}", Solution::divide(i32::MAX, 1));
    // println!("{}", Solution::divide(i32::MIN, 1));
    // println!("{}", Solution::divide(i32::MIN, -1));
    // println!("{}", Solution::divide(-10, 3));
    // println!("{}", Solution::divide(-10, -3));
    println!("{}", Solution::divide(-2147483648, -3));
}

struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        if divisor == i32::MIN {
            if dividend == i32::MIN {
                return 1;
            }
            return 0;
        }
        if dividend == i32::MIN {
            if divisor == -1 {
                return i32::MAX;
            }
            if divisor == 1 {
                return i32::MIN;
            }
            if divisor > 0 {
                let (s, _) = div(-(dividend + divisor), divisor);
                return -s - 1;
            } else {
                let (s, _) = div(-(dividend - divisor), -divisor);
                return s + 1;
            }
        }
        if dividend > 0 && divisor > 0 {
            let (s, _) = div(dividend, divisor);
            return s;
        } else if dividend > 0 && divisor < 0 {
            let (s, _) = div(dividend, -divisor);
            return -s;
        } else if dividend < 0 && divisor > 0 {
            let (s, _) = div(-dividend, divisor);
            return -s;
        } else {
            let (s, _) = div(-dividend, -divisor);
            return s;
        }
    }
}

fn div(dividend: i32, divisor: i32) -> (i32, i32) {
    if dividend < divisor {
        return (0, dividend);
    }
    if dividend == divisor {
        return (1, 0);
    }
    match divisor.checked_add(divisor) {
        Some(two_time) => {
            if dividend < two_time {
                return (1, dividend - divisor);
            }
            let (s, r) = div(dividend, two_time);
            let (u, v) = div(r, divisor);
            return (s + s + u, v);
        }
        None => {
            return (1, dividend - divisor);
        }
    }
}
