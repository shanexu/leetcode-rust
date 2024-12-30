fn main() {}

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut m: i32 = 1;
        for i in 0..points.len() {
            let x1 = points[i][0];
            let y1 = points[i][1];
            let mut line_map: HashMap<(i32, i32), i32> = HashMap::new();
            for j in (i + 1)..points.len() {
                let x2 = points[j][0];
                let y2 = points[j][1];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let slope = fraction(dx, dy);
                let value = line_map.entry(slope).and_modify(|v| *v += 1).or_insert(1);
                m = std::cmp::max(m, *value + 1);
            }
        }
        m
    }
}

fn fraction(mut numerator: i32, mut denominator: i32) -> (i32, i32) {
    if denominator == 0 {
        return (1, 0);
    }
    if numerator == 0 {
        return (0, 1);
    }
    let gcd = gcd(numerator, denominator);
    numerator /= gcd;
    denominator /= gcd;
    if denominator < 0 {
        numerator *= -1;
        denominator *= -1;
    }
    (numerator, denominator)
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
mod test {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(100, -30), 10);
    }

    #[test]
    fn test_fraction() {
        assert_eq!(fraction(30, 100), (3, 10));
        assert_eq!(fraction(30, -100), (-3, 10));
        assert_eq!(fraction(-30, -100), (3, 10));
        assert_eq!(fraction(-30, 100), (-3, 10));
        assert_eq!(fraction(300, 0), (1, 0));
        assert_eq!(fraction(0, -300), (0, 1));
    }
}
