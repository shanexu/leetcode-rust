fn main() {
    assert_eq!(Solution::find_final_value(vec![1, 2, 4, 8], 512), 512);
}

struct Solution;

impl Solution {
    pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
        nums.sort();
        while nums.binary_search(&original).is_ok() {
            original *= 2;
        }
        original
    }
}
