fn main() {
    assert_eq!(3, Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]));
    assert_eq!(3, Solution::maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]));
    assert_eq!(4, Solution::maximum_count(vec![0, 0, 5, 20, 66, 1314]));
    assert_eq!(4, Solution::maximum_count(vec![5, 20, 66, 1314]));
    assert_eq!(6, Solution::maximum_count(vec![-1, -2, -3, -4, -5, -6]));
    assert_eq!(
        6,
        Solution::maximum_count(vec![-1, -2, -3, -4, -5, -6, 0, 0])
    );
}

struct Solution;
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if nums[0] > 0 || nums[n - 1] < 0 {
            return nums.len() as i32;
        }
        if nums[0] == 0 && nums[n - 1] == 0 {
            return 0;
        }
        let mut size = n;
        let mut left = 0;
        if nums[0] == 0 {
            while size > 1 {
                let half = size / 2;
                let mid = left + half;
                if nums[mid] <= 0 {
                    left = mid;
                }
                size -= half;
            }
            return (n - left - 1) as i32;
        }
        if nums[n - 1] == 0 {
            while size > 1 {
                let half = size / 2;
                let mid = left + half;
                if nums[mid] < 0 {
                    left = mid;
                }
                size -= half;
            }
            return (left + 1) as i32;
        }
        while size > 1 {
            let half = size / 2;
            let mid = left + half;
            if nums[mid] < 0 {
                left = mid;
            }
            size -= half;
        }
        let neg = left + 1;
        size = n - left;
        while size > 1 {
            let half = size / 2;
            let mid = left + half;
            if nums[mid] <= 0 {
                left = mid;
            }
            size -= half;
        }
        let pos = n - left - 1;
        neg.max(pos) as i32
    }
}
