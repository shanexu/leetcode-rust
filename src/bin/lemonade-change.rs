fn main() {
    println!("{}", Solution::lemonade_change(vec![5,5,10,20,5,5,5,5,5,5,5,5,5,10,5,5,20,5,20,5]));
}

struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut n5 = 0;
        let mut n10 = 0;
        for b in bills {
            if b == 20 {
                if n10 >= 1 {
                    n10 -= 1;
                    if n5 >= 1 {
                        n5 -= 1;
                        continue;
                    }
                }
                if n5 >= 3 {
                    n5 -= 3;
                    continue;
                }
                return false;
            } else if b == 10 {
                if n5 >= 1 {
                    n5 -= 1;
                    n10 += 1;
                    continue;
                }
                return false;
            } else {
                n5 += 1;
            }
        }
        true
    }
}
