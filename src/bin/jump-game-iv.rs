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
use std::collections::HashSet;
use std::mem::swap;

impl Solution {
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
        let mut a_list = vec![0];
        let mut b_list = vec![0];
        let mut jump = 0;
        let mut visited: HashSet<usize> = HashSet::new();
        visited.insert(0);
        loop {
            for &i in a_list.iter() {
                let v = arr[i];
                for &j in map.get(&v).unwrap().iter() {
                    if j != i && !visited.contains(&j) {
                        visited.insert(j);
                        b_list.push(j);
                    }
                }
                if let Some(x) = map.get_mut(&v) {
                    x.clear();
                }
                if i == n - 1 {
                    return jump;
                }
                if i >= 1 && !visited.contains(&(i - 1)) {
                    visited.insert(i - 1);
                    b_list.push(i - 1);
                }
                if i < n - 1 && !visited.contains(&(i + 1)) {
                    visited.insert(i + 1);
                    b_list.push(i + 1);
                }
            }
            a_list.clear();
            swap(&mut a_list, &mut b_list);
            jump += 1;
        }
    }
}
