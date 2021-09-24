fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(Solution::can_jump(vec![0]), true);
    assert_eq!(Solution::can_jump(vec![1, 0]), true);
    assert_eq!(Solution::can_jump(vec![2, 0, 0]), true);
    assert_eq!(Solution::can_jump(vec![3, 0, 0, 0]), true);
    assert_eq!(Solution::can_jump(vec![2,0,1,0,1]), false);
}

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            if nums[i] == 0  && i != nums.len() - 1 {
                let mut ok = false;
                for j in (0..i).rev() {
                    let d = i - j;
                    if nums[j] > d as i32 {
                        ok = true;
                        break
                    }
                }
                if !ok {
                    return false;
                }
            }
        }
        true
    }
}
