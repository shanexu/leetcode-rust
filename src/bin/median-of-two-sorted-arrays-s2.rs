fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3])
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
    );
}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let m = nums1.len();
        let n = nums2.len();
        let mut left = 0;
        let mut right = m;
        while left <= right {
            let pa = (left + right) / 2;
            let pb = (m + n + 1) / 2 - pa;
            let mut max_left_a = i32::MIN;
            if pa != 0 {
                max_left_a = nums1[pa - 1];
            }
            let mut min_right_a = i32::MAX;
            if pa != m {
                min_right_a = nums1[pa];
            }
            let mut max_left_b = i32::MIN;
            if pb != 0 {
                max_left_b = nums2[pb - 1];
            }
            let mut min_right_b = i32::MAX;
            if pb != n {
                min_right_b = nums2[pb];
            }
            if max_left_a <= min_right_b && max_left_b <= min_right_a {
                if (m + n) % 2 == 0 {
                    let max_left = (max_left_a as f64).max(max_left_b as f64);
                    let min_right = (min_right_a as f64).min(min_right_b as f64);
                    return (max_left + min_right) / 2.0;
                } else {
                    return (max_left_a as f64).max(max_left_b as f64);
                }
            } else if max_left_a > min_right_b {
                right = pa - 1;
            } else {
                left = pa + 1;
            }
        }
        0.0
    }
}
