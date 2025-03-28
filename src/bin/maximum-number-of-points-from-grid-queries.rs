fn main() {
    println!(
        "{:?}",
        Solution::max_points(
            vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]],
            vec![5, 6, 2]
        )
    );
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let rows = grid.len();
        let cols = grid[0].len();
        let n = queries.len();
        let mut indices: Vec<(i32, usize)> = queries
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect::<Vec<(i32, usize)>>();
        indices.sort_unstable();
        let mut visited = vec![vec![false; cols]; rows];
        let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        heap.push((Reverse(grid[0][0]), 0, 0));
        let mut idx = 0usize;
        let mut ans = vec![0; n];
        while let Some((Reverse(v), i, j)) = heap.pop() {
            if visited[i][j] {
                continue;
            }
            while idx < n && v >= indices[idx].0 {
                idx += 1;
            }
            if idx == n {
                break;
            }
            visited[i][j] = true;
            ans[indices[idx].1] += 1;
            let mut dx = 0;
            let mut dy = 1;
            for _ in 0..4 {
                let new_row = i as i32 + dx;
                let new_col = j as i32 + dy;
                (dx, dy) = (-dy, dx);
                if new_row >= 0
                    && new_row < rows as i32
                    && new_col >= 0
                    && new_col < cols as i32
                    && !visited[new_row as usize][new_col as usize]
                {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;
                    heap.push((Reverse(grid[new_row][new_col]), new_row, new_col));
                }
            }
        }
        for i in 1..n {
            ans[indices[i].1] += ans[indices[i - 1].1];
        }
        ans
    }
}
