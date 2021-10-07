fn main() {
    assert_eq!(Solution::my_sqrt(1), 1);
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
}

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 1 {
            return 0;
        }
        let mut i = 1;
        let mut j = x / 2;
        let mut r = 1;
        while i <= j {
            let mid = i + (j - i) / 2;
            if mid == x / mid {
                return mid;
            } else if mid < x / mid {
                r = mid;
                i = mid + 1;
            } else {
                j = mid - 1;
            }
        }
        r
    }
}
