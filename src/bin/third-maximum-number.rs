fn main() {
    println!("{}", Solution::third_max(vec![3, 2, 1]));
    println!("{}", Solution::third_max(vec![1, 2]));
    println!("{}", Solution::third_max(vec![2, 2, 3, 1]));
    println!("{}", Solution::third_max(vec![5, 2, 4, 1, 3, 6, 0]));
}

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut m1 = None;
        let mut m2 = None;
        let mut m3 = None;
        let mut tmp: Option<i32>;
        for num in nums {
            (m1, tmp) = compare(m1, Some(num));
            (m2, tmp) = compare(m2, tmp);
            (m3, _) = compare(m3, tmp);
        }
        m3.unwrap_or(m1.unwrap())
    }
}

#[inline(always)]
fn compare(u: Option<i32>, v: Option<i32>) -> (Option<i32>, Option<i32>) {
    match (u, v) {
        (_, None) => (u, None),
        (None, _) => (v, None),
        (Some(x), Some(y)) => {
            if x > y {
                (u, v)
            } else if x < y {
                (v, u)
            } else {
                (u, None)
            }
        }
    }
}
