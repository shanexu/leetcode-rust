fn main() {
    assert_eq!(
        Solution::min_cost(vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2]
        ]),
        3
    );
    assert_eq!(
        Solution::min_cost(vec![
            vec![1, 1, 3],
            vec![2, 2, 2],
            vec![1, 1, 1],
            vec![2, 2, 2]
        ]),
        2
    );
    assert_eq!(Solution::min_cost(vec![vec![1, 2], vec![4, 3]]), 1);
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let num_rows = grid.len();
        let num_cols = grid[0].len();

        let mut visited = vec![vec![false; num_cols]; num_rows];
        let mut queue = VecDeque::with_capacity(num_rows * num_cols);

        queue.push_back((0, 0, 0));

        let directions = vec![(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((i, j, cost)) = queue.pop_front() {
            if i == num_rows - 1 && j == num_cols - 1 {
                return cost;
            }

            if visited[i][j] {
                continue;
            }

            visited[i][j] = true;

            for k in 1..=4 {
                let (new_x, new_y) = (i as i32 + directions[k].0, j as i32 + directions[k].1);

                if new_x >= 0 && new_x < num_rows as i32 && new_y >= 0 && new_y < num_cols as i32 {
                    if grid[i][j] == k as i32 {
                        queue.push_front((new_x as usize, new_y as usize, cost));
                    } else {
                        queue.push_back((new_x as usize, new_y as usize, cost + 1));
                    }
                }
            }
        }
        unreachable!()
    }
}
