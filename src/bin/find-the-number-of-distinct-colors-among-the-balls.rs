use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{:?}",
        Solution::query_results(4, vec_vec_i32![[1, 4], [2, 5], [1, 3], [3, 4]])
    );
}

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut balls: HashMap<i32, i32> = HashMap::new();
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let idx = q[0];
            let color = q[1];
            let color_point = balls.entry(idx).or_insert(0);
            let old_color = *color_point;
            *color_point = color;
            if old_color != 0 && old_color != color {
                let idx_set = map.get_mut(&old_color).unwrap();
                idx_set.remove(&idx);
                if idx_set.is_empty() {
                    map.remove(&old_color);
                }
            }
            if old_color != color {
                map.entry(color).or_insert(HashSet::new()).insert(idx);
            }
            ans.push(map.len() as i32);
        }
        ans
    }
}
