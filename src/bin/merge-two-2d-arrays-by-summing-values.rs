use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{:?}",
        Solution::merge_arrays(
            vec_vec_i32![[1, 2], [2, 3], [4, 5]],
            vec_vec_i32![[1, 4], [3, 2], [4, 1]]
        )
    );
}

struct Solution;

use std::cmp::Ordering::{Equal, Greater, Less};
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(nums1.len() + nums2.len());
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            match nums1[i][0].cmp(&nums2[j][0]) {
                Less => {
                    ans.push(nums1[i].clone());
                    i += 1;
                }
                Equal => {
                    ans.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                    i += 1;
                    j += 1;
                }
                Greater => {
                    ans.push(nums2[j].clone());
                    j += 1;
                }
            }
        }
        while i < nums1.len() {
            ans.push(nums1[i].clone());
            i += 1;
        }
        while j < nums2.len() {
            ans.push(nums2[j].clone());
            j += 1;
        }
        ans
    }
}
