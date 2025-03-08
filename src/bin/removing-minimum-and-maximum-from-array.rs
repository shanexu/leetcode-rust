fn main() {
    assert_eq!(
        5,
        Solution::minimum_deletions(vec![2, 10, 7, 5, 4, 1, 8, 6])
    );
}

struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 1;
        }
        let mut max = nums[0];
        let mut idx1 = 0;
        let mut min = nums[0];
        let mut idx2 = 0;
        for i in 1..n {
            let num = nums[i];
            if max < num {
                max = num;
                idx1 = i;
            }
            if min > num {
                min = num;
                idx2 = i;
            }
        }
        if idx1 > idx2 {
            std::mem::swap(&mut idx1, &mut idx2);
        }
        let mut ans = idx2 + 1;
        ans = ans.min(idx1 + 1 + n - idx2);
        ans = ans.min(n - idx1);
        ans as i32
    }
}
