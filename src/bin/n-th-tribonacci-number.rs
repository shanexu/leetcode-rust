fn main() {
    println!("{}", Solution::tribonacci(0));
    println!("{}", Solution::tribonacci(1));
    println!("{}", Solution::tribonacci(2));
    println!("{}", Solution::tribonacci(3));
    println!("{}", Solution::tribonacci(4));
    println!("{}", Solution::tribonacci(5));
    println!("{}", Solution::tribonacci(25));
}

struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut n = n;
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        while n > 0 {
            let t = c;
            c = a + b + c;
            a = b;
            b = t;
            n -= 1;
        }
        a
    }
}
