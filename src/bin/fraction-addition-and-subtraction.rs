fn main() {
    println!("{}", Solution::fraction_addition(String::from("1/3-1/2")));
}

struct Solution;

use std::fmt::{Display, Formatter};
use std::ops;
impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let bs = expression.as_bytes();
        let mut state = ParseState::ReadDenominator;
        let mut sign: i32 = 1;
        let mut numerator = 0;
        let mut denominator = 0;
        let mut ans : Fraction = Fraction::new(0 ,1);
        if bs[0] == b'-' {
            denominator = 1;
        } else if bs[0] == b'+' {
            denominator = 1;
        } else {
            state = ParseState::ReadNumerator;
        }
        for &b in bs {
            if state == ParseState::ReadNumerator {
                if b == b'/' {
                    numerator *= sign;
                    sign = 1;
                    state = ParseState::ReadDenominator;
                } else {
                    numerator *= 10;
                    numerator += (b - b'0') as i32;
                }
            } else if state == ParseState::ReadDenominator {
                if b == b'+' {
                    sign = 1;
                    state = ParseState::ReadNumerator;
                    ans += Fraction::new(numerator, denominator);
                    numerator = 0;
                    denominator = 0;
                } else if b == b'-' {
                    sign = -1;
                    state = ParseState::ReadNumerator;
                    ans += Fraction::new(numerator, denominator);
                    numerator = 0;
                    denominator = 0;
                } else {
                    denominator *= 10;
                    denominator += (b - b'0') as i32;
                }
            }
        }
        ans += Fraction::new(numerator, denominator);
        format!("{}", ans)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ParseState {
    ReadNumerator,
    ReadDenominator
}

#[derive(Debug, Eq, PartialEq)]
struct Fraction(i32, i32);

impl Fraction {
    fn new(mut numerator: i32, mut denominator: i32) -> Fraction {
        if numerator == 0 && denominator == 0 {
            return Fraction(0, 0);
        }
        if numerator == 0 {
            return Fraction(0, 1);
        }
        if denominator == 0 {
            return Fraction(numerator.signum(), 0);
        }
        let sign = numerator.signum() * denominator.signum();
        numerator = numerator.abs();
        denominator = denominator.abs();
        let gcd = gcd(numerator, denominator);
        numerator /= gcd;
        denominator /= gcd;
        Fraction(sign * numerator, denominator)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 != 0
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl ops::Add<Fraction> for Fraction {
    type Output = Self;

    fn add(self, rhs: Fraction) -> Self::Output {
        if self.is_zero() {
            return rhs;
        }
        Fraction::new(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
    }
}

impl ops::AddAssign<Fraction> for Fraction {
    fn add_assign(&mut self, rhs: Fraction) {
        if self.is_zero() {
            *self = rhs;
        } else {
            *self = Fraction::new(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
        }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fraction_addition() {
        let f1 = Fraction::new(1, 2) + Fraction::new(3, 4);
        println!("{}", f1);
        assert_eq!(f1, Fraction(5, 4));
    }
}
