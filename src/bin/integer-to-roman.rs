/*
Symbol       Value
I             1
IV            4
V             5
IX            9
X             10
XL            40
L             50
XC            90
C             100
CD            400
D             500
CM            900
M             1000
*/

fn main() {
    println!("{}", Solution::int_to_roman(3));
    println!("{}", Solution::int_to_roman(4));
    println!("{}", Solution::int_to_roman(9));
    println!("{}", Solution::int_to_roman(58));
    println!("{}", Solution::int_to_roman(1994));
}

struct Solution {}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut vec = Vec::new();
        let mut r = num;

        if r >= 1000 {
            let n = r / 1000;
            for _ in 0..n {
                vec.push(b'M');
            }
            r = r - n * 1000
        }
        if r >= 900 {
            vec.push(b'C');
            vec.push(b'M');
            r = r - 900;
        }
        if r >= 500 {
            vec.push(b'D');
            r = r - 500;
        }
        if r >= 400 {
            vec.push(b'C');
            vec.push(b'D');
            r = r - 400;
        }
        if r >= 100 {
            let n = r / 100;
            for _ in 0..n {
                vec.push(b'C')
            }
            r = r - n * 100;
        }
        if r >= 90 {
            vec.push(b'X');
            vec.push(b'C');
            r = r - 90;
        }
        if r >= 50 {
            vec.push(b'L');
            r = r - 50;
        }
        if r >= 40 {
            vec.push(b'X');
            vec.push(b'L');
            r = r - 40;
        }
        if r >= 10 {
            let n = r / 10;
            for _ in 0..n {
                vec.push(b'X');
            }
            r = r - n * 10;
        }
        if r >= 9 {
            vec.push(b'I');
            vec.push(b'X');
            r = r - 9;
        }
        if r >= 5 {
            vec.push(b'V');
            r = r - 5;
        }
        if r >= 4 {
            vec.push(b'I');
            vec.push(b'V');
            r = r - 4;
        }
        if r > 0 {
            for _ in 0..r {
                vec.push(b'I');
            }
        }
        String::from_utf8(vec).unwrap()
    }
}
