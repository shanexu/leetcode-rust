fn main() {
    println!(
        "{:?}",
        Solution::merge_similar_items(
            vec_vec_i32![[1, 1], [3, 2], [2, 3]],
            vec_vec_i32![[2, 1], [3, 2], [1, 3]]
        )
    );
}

struct Solution;

use leetcode_rust::vec_vec_i32;
use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn merge_similar_items(
        mut items1: Vec<Vec<i32>>,
        mut items2: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(items1.len() + items2.len());
        items1.sort_by(|a, b| a[0].cmp(&b[0]));
        items2.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut i = 0;
        let mut j = 0;
        while i < items1.len() && j < items2.len() {
            match items1[i][0].cmp(&items2[j][0]) {
                Less => {
                    ans.push(items1[i].clone());
                    i += 1;
                }
                Equal => {
                    ans.push(vec![items1[i][0], items1[i][1] + items2[j][1]]);
                    i += 1;
                    j += 1;
                }
                Greater => {
                    ans.push(items2[j].clone());
                    j += 1;
                }
            }
        }
        while i < items1.len() {
            ans.push(items1[i].clone());
            i += 1;
        }
        while j < items2.len() {
            ans.push(items2[j].clone());
            j += 1;
        }
        ans
    }
}
