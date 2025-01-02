fn main() {}

struct Solution;

impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let (i, &v) = nums.iter().enumerate().max_by_key(|(_, &val)| val).unwrap();
        if nums.iter().all(|&x| x == v || v >= 2 * x) {
            i as i32
        } else {
            -1
        }
    }
}
