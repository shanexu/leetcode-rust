use leetcode_rust::vec_vec_i32;

fn main() {
    // println!(
    //     "{:?}",
    //     Solution::hit_bricks(
    //         vec_vec_i32![[1, 0, 0, 0], [1, 1, 1, 0]],
    //         vec_vec_i32![[1, 0]]
    //     )
    // );
    println!(
        "{:?}",
        Solution::hit_bricks(
            vec_vec_i32![[1], [1], [1], [1], [1]],
            vec_vec_i32![[3, 0], [4, 0], [1, 0], [2, 0], [0, 0]]
        )
    );
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut queue = VecDeque::new();
        for (i, h) in hits.iter().enumerate() {
            grid[h[0] as usize][h[1] as usize] = -(i as i32) - 1;
        }
        for i in 0..n {
            if grid[0][i] == 1 {
                queue.push_back((0, i));
                while let Some((x, y)) = queue.pop_front() {
                    if grid[x][y] == 1 {
                        grid[x][y] = 2;
                        let mut dx = 1;
                        let mut dy = 0;
                        for _ in 0..4 {
                            let new_x = x as isize + dx;
                            let new_y = y as isize + dy;
                            (dx, dy) = (dy, -dx);
                            if new_x >= 0 && new_y >= 0 {
                                let new_x = new_x as usize;
                                let new_y = new_y as usize;
                                if new_x < m && new_y < n && grid[new_x][new_y] == 1 {
                                    queue.push_back((new_x, new_y));
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("{:?}", grid);
        let mut ans = vec![0; hits.len()];
        for t in (0..hits.len()).rev() {
            let h = &hits[0];
            let i = h[0] as usize;
            let j = h[1] as usize;
            let mut max_hit = grid[i][j];
            if max_hit >= 0 {
                continue;
            }
            let mut c = 0;
            queue.push_back((i, j));
            while let Some((x, y)) = queue.pop_front() {
                if grid[x][y] == 0 {
                    continue;
                }
                if grid[x][y] == 2 {
                    continue;
                }
                if grid[x][y] < 0 {
                    max_hit = max_hit.max(grid[x][y]);
                }
                c += 1;
                grid[x][y] = 2;
                let mut dx = 1;
                let mut dy = 0;
                for _ in 0..4 {
                    let new_x = x as isize + dx;
                    let new_y = y as isize + dy;
                    (dx, dy) = (dy, -dx);
                    if new_x >= 0 && new_y >= 0 {
                        let new_x = new_x as usize;
                        let new_y = new_y as usize;
                        if new_x < m && new_y < n {
                            queue.push_back((new_x, new_y));
                        }
                    }
                }
            }
            println!("xxx {} {}", c, max_hit);
            ans[t] = c - 1;
        }
        ans
    }
}
