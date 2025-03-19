fn main() {
    println!("{}", Solution::min_operations(vec![0, 1, 1]));
    println!("{}", Solution::min_operations(vec![0, 1, 1, 1, 0, 0]));
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut flip1 = 0;
        let mut flip2 = 0;
        let mut ans = 0;
        for num in nums {
            if num ^ flip1 == 0 {
                ans += 1;
                flip1 = flip2 ^ 1;
                flip2 = 1;
            } else {
                flip1 = flip2;
                flip2 = 0;
            }
        }
        if flip1 == 0 && flip2 == 0 {
            ans
        } else {
            -1
        }
    }
}
