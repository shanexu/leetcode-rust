fn main() {
    println!("{}", Solution::my_pow(2.0, 10));
    println!("{}", Solution::my_pow(2.1, 3));
    println!("{}", Solution::my_pow(2.0, -2));
    println!("{}", Solution::my_pow(0.999999999, -2147483648))
}

struct Solution;

/*
*/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == i32::MIN {
            let v = pow(1.0 / x, 1 << 30, 1.0);
            return v * v;
        }
        if n < 0 {
            return pow(1.0 / x, -n, 1.0);
        }
        pow(x, n, 1.0)
    }
}

fn pow(x: f64, n: i32, r: f64) -> f64 {
    if n == 0 {
        return r;
    }
    if n == 1 {
        return x * r;
    }
    let half = n / 2;
    pow(x * x, half, if n == half * 2 { r } else { r * x })
}
