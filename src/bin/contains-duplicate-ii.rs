fn main() {
    println!(
        "{}",
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3)
    );
}

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        for (j, &v) in nums.iter().enumerate() {
            if let Some(i) = map.insert(v, j) {
                if j - i <= k {
                    return true;
                }
            }
        }
        false
    }
}
