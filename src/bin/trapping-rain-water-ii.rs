fn main() {
    println!(
        "{}",
        Solution::trap_rain_water(vec_vec_i32![
            [1, 4, 3, 1, 3, 2],
            [3, 2, 1, 3, 2, 4],
            [2, 3, 3, 2, 3, 1]
        ])
    );
    println!(
        "{}",
        Solution::trap_rain_water(vec_vec_i32![
            [3, 3, 3, 3, 3],
            [3, 2, 2, 2, 3],
            [3, 2, 1, 2, 3],
            [3, 2, 2, 2, 3],
            [3, 3, 3, 3, 3]
        ])
    );
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use leetcode_rust::vec_vec_i32;
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let rows = height_map.len();
        let cols = height_map[0].len();
        if rows <= 2 || cols <= 2 {
            return 0;
        }
        let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        let mut visited = vec![vec![false; cols]; rows];
        for (j, &h) in height_map[0].iter().enumerate() {
            heap.push((Reverse(h), 0, j));
            visited[0][j] = true;
        }
        for (j, &h) in height_map[rows - 1].iter().enumerate() {
            heap.push((Reverse(h), rows - 1, j));
            visited[rows - 1][j] = true;
        }
        for i in 1..rows - 1 {
            heap.push((Reverse(height_map[i][0]), i, 0));
            visited[i][0] = true;
            heap.push((Reverse(height_map[i][cols - 1]), i, cols - 1));
            visited[i][cols - 1] = true;
        }
        let mut ans = 0;
        while let Some((Reverse(h), i, j)) = heap.pop() {
            for k in 0..4 {
                let new_row = i as i32 + DIRECTIONS[k];
                let new_col = j as i32 + DIRECTIONS[k + 1];
                if new_row >= 0
                    && new_row < rows as i32
                    && new_col >= 0
                    && new_col < cols as i32
                    && !visited[new_row as usize][new_col as usize]
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    let new_h = height_map[new_row][new_col];
                    ans += 0.max(h - new_h);
                    visited[new_row][new_col] = true;
                    heap.push((Reverse(h.max(new_h)), new_row, new_col));
                }
            }
        }
        ans
    }
}

const DIRECTIONS: &[i32] = &[-1, 0, 1, 0, -1];
