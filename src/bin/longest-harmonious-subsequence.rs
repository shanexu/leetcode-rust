fn main() {
    assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4, 5, 6, 7, 8]), 2);
    assert_eq!(Solution::find_lhs(vec![1, 1, 2, 2, 3, 3, 4, 4]), 4);
}

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
