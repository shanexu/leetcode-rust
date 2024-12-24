fn main() {
    // println!(
    //     "{:?}",
    //     Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5)
    // );
    // println!("{:?}", Solution::max_number(vec![6, 7], vec![6, 0, 4], 5));
    // println!("{:?}", Solution::max_number(vec![3, 9], vec![8, 9], 3));
    println!(
        "{:?}",
        Solution::max_number(vec![2, 5, 6, 4, 4, 0], vec![7, 3, 8, 0, 6, 5, 7, 6, 2], 15)
    );
    println!("{:?}", Solution::max_number(vec![6, 7], vec![6, 0, 4], 5));
}

struct Solution;

impl Solution {
    pub fn max_number(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let n1 = nums1.len();
        let n2 = nums2.len();
        let k = k as usize;
        let min_s1 = if k > n2 { k - n2 } else { 0 };
        let max_s1 = std::cmp::min(k, n1);
        let mut ans = vec![0; k];
        for s1 in min_s1..=max_s1 {
            let v1 = max_sub(&nums1, s1);
            let v2 = max_sub(&nums2, k - s1);
            let v = merge(v1, v2);
            if gt(&v, &ans) {
                ans = v;
            }
        }
        ans
    }
}

fn max_sub(nums: &Vec<i32>, len: usize) -> Vec<i32> {
    let mut stack = Vec::with_capacity(len);
    let n = nums.len();
    for (i, &v) in nums.iter().enumerate() {
        while !stack.is_empty() && stack[stack.len() - 1] < v && (n - i > len - stack.len()) {
            stack.pop();
        }
        if stack.len() < len {
            stack.push(v);
        }
    }
    stack
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if gt(&left[left_index..], &right[right_index..]) {
            merged.push(left[left_index]);
            left_index += 1;
        } else {
            merged.push(right[right_index]);
            right_index += 1;
        }
    }

    // Copy remaining elements from left or right
    merged.extend_from_slice(&left[left_index..]);
    merged.extend_from_slice(&right[right_index..]);

    merged
}

fn gt(nums1: &[i32], nums2: &[i32]) -> bool {
    let mut i = 0;
    while i < nums1.len() && i < nums2.len() {
        match &nums1[i].cmp(&nums2[i]) {
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Greater => return true,
            _ => i += 1,
        }
    }
    i < nums1.len()
}
