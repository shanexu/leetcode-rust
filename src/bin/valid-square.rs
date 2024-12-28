fn main() {
    println!(
        "{}",
        Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1])
    );
}

struct Solution;

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut a = &p1;
        let mut b = &p2;
        let mut c = &p3;
        let d = &p4;
        let mut ab2 = square(b[0] - a[0]) + square(b[1] - a[1]);
        let mut bc2 = square(c[0] - b[0]) + square(c[1] - b[1]);
        let mut ca2 = square(a[0] - c[0]) + square(a[1] - c[1]);

        if ab2 == 0 || bc2 == 0 || ca2 == 0 {
            return false;
        }

        if ab2 > ca2 {
            std::mem::swap(&mut ab2, &mut ca2);
            std::mem::swap(&mut c, &mut b);
        } else if bc2 > ca2 {
            std::mem::swap(&mut bc2, &mut ca2);
            std::mem::swap(&mut a, &mut b);
        }

        if 2 * ab2 != ca2 {
            return false;
        }
        if ab2 != bc2 {
            return false;
        }

        let da2 = square(a[0] - d[0]) + square(a[1] - d[1]);
        if da2 != ab2 {
            return false;
        }
        let dc2 = square(c[0] - d[0]) + square(c[1] - d[1]);
        if dc2 != bc2 {
            return false;
        }
        let db2 = square(b[0] - d[0]) + square(b[1] - d[1]);
        if db2 != ca2 {
            return false;
        }

        true
    }
}

#[inline]
fn square(x: i32) -> i32 {
    x * x
}
