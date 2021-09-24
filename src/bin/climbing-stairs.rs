/*
n=1 1
n=2 2
n=3 3
111
12
21
n=4 5
1111
112
211
22
121
n=5 8
11111
1121
2111
221
1211
1112
122
212

f(n) = f(n-1)+f(n-2)
*/

fn main() {
    println!("{}", Solution::climb_stairs(1));
    println!("{}", Solution::climb_stairs(2));
    println!("{}", Solution::climb_stairs(3));
    println!("{}", Solution::climb_stairs(4));
    println!("{}", Solution::climb_stairs(5));
}

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 1;
        let mut b = 1;
        let mut n = n;
        while n > 0 {
            let t = b;
            b = a+b;
            a = t;
            n = n-1;
        }
        a
    }
}
