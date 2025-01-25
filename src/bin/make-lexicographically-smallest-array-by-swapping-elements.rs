fn main() {
    println!("{:?}", Solution::lexicographically_smallest_array(vec![1,7,6,18,2,1], 3));
}

struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut map: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
        let mut ans = vec![0; n];
        for (i, &num) in nums.iter().enumerate() {
            map.entry(num).or_insert(vec![]).push(i);
        }
        let mut prev = 0;
        let mut ns = vec![];
        let mut is = vec![];
        for (num, js) in map {
            if num - prev > limit {
                is.sort();
                for k in 0..is.len() {
                    ans[is[k]] = ns[k];
                }
                ns.clear();
                is.clear()
            }
            for j in js {
                is.push(j);
                ns.push(num);
            }
            prev = num;
        }
        is.sort();
        for k in 0..is.len() {
            ans[is[k]] = ns[k];
        }
        ans
    }
}