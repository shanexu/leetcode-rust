fn main() {}

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut x = 0;
        for &num in nums.iter() {
            x ^= num;
        }
        for i in 0..32 {
            if (x >> i) & 1 == 1 {
                let mut u = 0;
                let mut v = 0;
                for &num in nums.iter() {
                    if num >> i & 1 == 0 {
                        u ^= num;
                    } else {
                        v ^= num;
                    }
                }
                return vec![u, v];
            }
        }
        unreachable!();
    }
}
