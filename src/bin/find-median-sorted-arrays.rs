fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2])
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4])
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0])
    );
    println!("{}", Solution::find_median_sorted_arrays(vec![], vec![1]));
    println!("{}", Solution::find_median_sorted_arrays(vec![2], vec![]));
    println!("###");
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![-5, 3, 6, 12, 15], vec![-12, -10, -6, -3, 4, 10])
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![2, 3, 5, 8], vec![10, 12, 14, 16, 18, 20])
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19]
        )
    );
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5, 6])
    );
}

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            find_median_sorted_arrays_main(&nums2, 0, nums2.len(), &nums1, 0, nums1.len())
        } else {
            find_median_sorted_arrays_main(&nums1, 0, nums1.len(), &nums2, 0, nums2.len())
        }
    }
}

fn find_median_sorted_arrays_main(
    nums1: &Vec<i32>,
    nums1_begin: usize,
    nums1_length: usize,
    nums2: &Vec<i32>,
    nums2_begin: usize,
    nums2_length: usize,
) -> f64 {
    let mut nums1_begin = nums1_begin;
    let mut nums1_length = nums1_length;
    let mut nums2_begin = nums2_begin;
    let mut nums2_length = nums2_length;

    loop {
        if nums1_length == 0 {
            return find_median(nums2, nums2_begin, nums2_length);
        }
        if nums1_length == 1 {
            if nums2_length == 1 {
                return median2(nums1[nums1_begin] as f64, nums2[nums2_begin] as f64);
            } else if nums2_length % 2 == 1 {
                let mid_idx = nums2_begin + nums2_length / 2;
                return median2(
                    nums2[mid_idx] as f64,
                    median3(
                        nums1[nums1_begin] as f64,
                        nums2[mid_idx - 1] as f64,
                        nums2[mid_idx + 1] as f64,
                    ),
                );
            } else {
                let mid_idx = nums2_begin + nums2_length / 2;
                return median3(
                    nums1[nums1_begin] as f64,
                    nums2[mid_idx - 1] as f64,
                    nums2[mid_idx] as f64,
                );
            }
        }
        if nums1_length == 2 {
            if nums2_length == 2 {
                return median4(
                    nums1[nums1_begin] as f64,
                    nums1[nums1_begin + 1] as f64,
                    nums2[nums2_begin] as f64,
                    nums2[nums2_begin + 1] as f64,
                );
            } else if nums2_length % 2 == 1 {
                let mid_idx = nums2_begin + nums2_length / 2;
                return median3(
                    max(nums1[nums1_begin] as f64, nums2[mid_idx - 1] as f64),
                    nums2[mid_idx] as f64,
                    min(nums1[nums1_begin + 1] as f64, nums2[mid_idx + 1] as f64),
                );
            } else {
                let mid_idx = nums2_begin + nums2_length / 2;
                return median4(
                    max(nums1[nums1_begin] as f64, nums2[mid_idx - 2] as f64),
                    nums2[mid_idx - 1] as f64,
                    nums2[mid_idx] as f64,
                    min(nums1[nums1_begin + 1] as f64, nums2[mid_idx + 1] as f64),
                );
            }
        }
        let nums1_mid_idx = nums1_begin + (nums1_length - 1) / 2;
        let nums2_mid_idx = nums2_begin + (nums2_length - 1) / 2;
        if nums1[nums1_mid_idx] <= nums2[nums2_mid_idx] {
            nums1_length = nums1_length - (nums1_mid_idx - nums1_begin);
            nums2_length = nums2_length - (nums1_mid_idx - nums1_begin);
            nums1_begin = nums1_mid_idx;
        } else {
            nums1_length = nums1_length - (nums1_mid_idx - nums1_begin);
            nums2_length = nums2_length - (nums1_mid_idx - nums1_begin);
            nums2_begin = nums2_begin + (nums1_mid_idx - nums1_begin);
        }
    }
}

fn find_median(nums: &Vec<i32>, nums_begin: usize, length: usize) -> f64 {
    let mid_idx = nums_begin + length / 2;
    if length % 2 == 0 {
        median2(nums[mid_idx - 1] as f64, nums[mid_idx] as f64)
    } else {
        nums[mid_idx] as f64
    }
}

fn median2(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn median3(a: f64, b: f64, c: f64) -> f64 {
    let mut a = a;
    let mut b = b;
    if a > b {
        let t = a;
        a = b;
        b = t;
    }
    if c <= a {
        a
    } else if c <= b {
        c
    } else {
        b
    }
}

fn median4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    if a > b {
        let t = a;
        a = b;
        b = t;
    }
    if a > c {
        let t = a;
        a = c;
        c = t;
    }
    if b > c {
        let t = b;
        b = c;
        c = t;
    }
    if d <= a {
        median2(a, b)
    } else if d <= c {
        median2(b, d)
    } else {
        median2(b, c)
    }
}

fn max(a: f64, b: f64) -> f64 {
    if a < b {
        b
    } else {
        a
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a <= b {
        a
    } else {
        b
    }
}
