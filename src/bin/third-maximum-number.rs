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
        for num in nums {
            match (m1, m2, m3) {
                (None, _, _) => {
                    m1 = Some(num);
                }
                (Some(v1), None, None) => {
                    if num > v1 {
                        m1 = Some(num);
                        m2 = Some(v1);
                    } else if num < v1 {
                        m2 = Some(num);
                    }
                }
                (Some(v1), Some(v2), None) => {
                    if num > v1 {
                        m1 = Some(num);
                        m2 = Some(v1);
                        m3 = Some(v2);
                    } else if num < v1 && num > v2 {
                        m2 = Some(num);
                        m3 = Some(v2);
                    } else if num < v2 {
                        m3 = Some(num);
                    }
                }
                (Some(v1), Some(v2), Some(v3)) => {
                    if num > v1 {
                        m1 = Some(num);
                        m2 = Some(v1);
                        m3 = Some(v2);
                    } else if num < v1 && num > v2 {
                        m2 = Some(num);
                        m3 = Some(v2);
                    } else if num < v2 && num > v3 {
                        m3 = Some(num);
                    }
                }
                _ => unreachable!(),
            }
        }
        m3.unwrap_or(m1.unwrap())
    }
}
