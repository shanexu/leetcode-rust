fn main() {
    assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5), true);
    assert_eq!(Solution::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0), true);
    assert_eq!(Solution::can_reach(vec![3,0,2,1,2], 2), false);
}

struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut arr = arr;
        can_reach(&mut arr, start)
    }
}

fn can_reach(arr: &mut Vec<i32>, start: i32) -> bool {
    let v = arr[start as usize];
    if v == 0 {
        return true;
    }
    if v < 0 {
        return false;
    }
    arr[start as usize] = -v;
    let forward = start + v;
    if (forward as usize) < arr.len() {
        if can_reach(arr, forward) {
            arr[forward as usize] = -arr[forward as usize];
            return true;
        } else {
            arr[forward as usize] = -arr[forward as usize];
        }
    }
    let backward = start - v;
    if backward >= 0 {
        if can_reach(arr, backward) {
            arr[backward as usize] = -arr[backward as usize];
            return true;
        } else {
            arr[backward as usize] = -arr[backward as usize];
        }
    }
    return false;
}
