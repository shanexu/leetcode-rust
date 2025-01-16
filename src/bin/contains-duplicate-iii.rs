fn main() {}

struct Solution;

use std::collections::BTreeMap;
use std::ops::Bound::Included;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let mut tree_map: BTreeMap<i32, i32> = BTreeMap::new();
        let index_diff = index_diff as usize;
        for (i, &num ) in nums.iter().enumerate() {
            for _ in tree_map.range((Included(num - value_diff), Included(num + value_diff))) {
                return true;
            }
            *tree_map.entry(num).or_insert(0) += 1;
            if i >= index_diff {
                let remove = nums[i - index_diff];
                let v = tree_map.get_mut(&remove).unwrap();
                *v -= 1;
                if *v == 0 {
                    tree_map.remove(&remove);
                }
            }
        }
        false
    }
}