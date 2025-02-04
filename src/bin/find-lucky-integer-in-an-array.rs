fn main() {
    println!("{}", Solution::find_lucky(vec![2, 2, 3, 4]));
    println!("{}", Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]));
    println!("{}", Solution::find_lucky(vec![2, 2, 2, 3, 3]));
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for num in arr {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut ans = -1;
        for (&k, &v) in map.iter() {
            if k == v {
                ans = ans.max(k);
            }
        }
        ans
    }
}
