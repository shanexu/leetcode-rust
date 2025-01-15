fn main() {}
struct Solution;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut ans = 0;
        for (&k, &v) in map.iter() {
            if let Some(&u) = map.get(&(k + 1)) {
                ans = ans.max(v + u);
            }
        }
        ans
    }
}
