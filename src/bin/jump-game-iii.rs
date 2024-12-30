fn main() {
    assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
    assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
    assert_eq!(Solution::can_reach(vec![3, 0, 2, 1, 2], 2), false);
}

struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, s: i32) -> bool {
        let mut arr = arr;
        let mut start = vec![s];
        loop {
            match start.pop() {
                None => return false,
                Some(s) => {
                    let v = arr[s as usize];
                    if v == 0 {
                        return true;
                    }
                    if v < 0 {
                        continue;
                    }
                    arr[s as usize] = -v;
                    let forward = s + v;
                    if (forward as usize) < arr.len() {
                        start.push(forward);
                    }
                    let backward = s - v;
                    if backward >= 0 {
                        start.push(backward);
                    }
                }
            }
        }
    }
}
