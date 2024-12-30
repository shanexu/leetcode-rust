fn main() {
    println!("{}", Solution::fib(0));
    println!("{}", Solution::fib(1));
    println!("{}", Solution::fib(2));
    println!("{}", Solution::fib(3));
    println!("{}", Solution::fib(4));
    println!("{}", Solution::fib(5));
}

struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut n = n;
        while n > 0 {
            let t = b;
            b = a + b;
            a = t;
            n = n - 1;
        }
        a
    }
}
