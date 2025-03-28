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
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let mut i = 0;
        let mut j = 0;
        let mut prev = 0;
        let mut curr = 0;
        for _ in 0..=(m + n) / 2 {
            prev = curr;
            if i < m && j < n {
                if nums1[i] < nums2[j] {
                    curr = nums1[i];
                    i += 1;
                } else {
                    curr = nums2[j];
                    j += 1;
                }
            } else if i < m {
                curr = nums1[i];
                i += 1;
            } else if j < n {
                curr = nums2[j];
                j += 1;
            }
        }
        if (m + n) % 2 == 1 {
            curr as f64
        } else {
            (curr + prev) as f64 / 2.0
        }
    }
}
