fn main() {
    println!("{}", Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10));
}

struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut remains = vec![0; k as usize];
        for v in arr {
            let mut r = v % k;
            if r < 0 {
                r += k;
            }
            remains[r as usize] += 1;
        }
        if remains[0] & 1 != 0 {
            return false;
        }
        let k = k as usize;
        for i in 1..k {
            if i == k - i {
                if remains[i] & 1 != 0 {
                    return false;
                }
            }
            if i >= k - i {
                break;
            }
            if remains[i] != remains[k - i] {
                return false;
            }
        }
        true
    }
}
