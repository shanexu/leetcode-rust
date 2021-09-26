fn main() {
    assert_eq!(
        Solution::min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404]),
        3
    );
    assert_eq!(Solution::min_jumps(vec![7]), 0);
    assert_eq!(Solution::min_jumps(vec![7, 6, 9, 6, 9, 6, 9, 7]), 1);
    assert_eq!(Solution::min_jumps(vec![6, 1, 9]), 2);
    assert_eq!(
        Solution::min_jumps(vec![11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13]),
        3
    );
}

struct Solution;

use std::collections::HashMap;

impl Solution {
    /*
    这个算法有问题的，没有考虑，穿梭带来的回溯
    */
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
        for i in 0..arr.len() {
            let v = arr[i];
            if let Some(x) = map.get_mut(&v) {
                x.push(i);
            } else {
                map.insert(v, vec![i]);
            }
        }
        let n = arr.len();
        let mut rs = vec![-1; n];
        rs[0] = 0;
        for i in 0..n {
            if rs[i] == -1 {
                continue;
            }
            if i + 1 < n {
                rs[i + 1] = min(rs[i + 1], rs[i] + 1);
            }
            if i > 0 {
                rs[i - 1] = min(rs[i - 1], rs[i] + 1);
            }
            let v = arr[i];
            for &u in map.get(&v).unwrap().iter() {
                if u == i {
                    continue;
                }
                rs[u] = min(rs[u], rs[i] + 1);
            }
            println!("{} {:?}", i, rs);
        }
        rs[n - 1]
    }
}

fn min(a: i32, b: i32) -> i32 {
    if a == -1 {
        return b;
    }
    if a < b {
        return a;
    } else {
        return b;
    }
}
