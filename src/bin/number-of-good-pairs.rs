fn main() {}

struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut map = std::collections::HashMap::new();
        for num in nums {
            map.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }
        for (&_, &v) in map.iter() {
            ans += v * (v - 1) / 2;
        }
        ans
    }
}
