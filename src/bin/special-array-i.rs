fn main() {
    println!("{}", Solution::is_array_special(vec![1]));
    println!("{}", Solution::is_array_special(vec![2, 1, 4]));
    println!("{}", Solution::is_array_special(vec![4, 3, 1, 6]));
    println!("{}", (3 ^ 1) & 1);
}

struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut p = nums[0];
        for i in 1..n {
            let q = nums[i];
            if (p ^ q) & 1 == 0 {
                return false;
            }
            p = q;
        }
        true
    }
}
