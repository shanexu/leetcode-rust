fn main() {
    println!(
        "{}",
        Solution::lemonade_change(vec![
            5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5
        ])
    );
}

struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut n5 = 0;
        let mut n10 = 0;
        for b in bills {
            if b == 5 {
                n5 += 1;
            } else if b == 10 && n5 >= 1 {
                n5 -= 1;
                n10 += 1;
            } else if b == 20 && n10 >= 1 && n5 >= 1 {
                n10 -= 1;
                n5 -= 1;
            } else if b == 20 && n5 >= 3 {
                n5 -= 3;
            } else {
                return false;
            }
        }
        true
    }
}

struct Solution2;

impl Solution2 {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut n5 = 0;
        let mut n10 = 0;
        for b in bills {
            match (b, n5, n10) {
                (5, _, _) => n5 += 1,
                (10, 1.., _) => {
                    n10 += 1;
                    n5 -= 1;
                }
                (20, 1.., 1..) => {
                    n10 -= 1;
                    n5 -= 1;
                }
                (20, 3.., 0) => n5 -= 3,
                _ => return false,
            }
        }
        true
    }
}
