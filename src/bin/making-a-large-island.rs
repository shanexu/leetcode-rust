fn main() {
    println!("{}", Solution::largest_island(vec![vec![1, 0], vec![0, 1]]));
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;
        let mut visited: Vec<Vec<usize>> = vec![vec![0; n]; n];
        let mut groups: Vec<i32> = vec![0];
        let mut queue = VecDeque::new();
        for i in 0..n {
            for j in 0..n {
                if visited[i][j] == 0 && grid[i][j] == 1 {
                    groups.push(0);
                    let g = groups.len() - 1;
                    let mut c = 0;
                    queue.push_back((i, j));
                    while let Some((p, q)) = queue.pop_front() {
                        if visited[p][q] != 0 {
                            continue;
                        }
                        visited[p][q] = g;
                        c += 1;
                        let mut dx = 0;
                        let mut dy = 1;
                        for _ in 0..4 {
                            let new_p = p as i32 + dx;
                            let new_q = q as i32 + dy;
                            (dx, dy) = (dy, -dx);
                            if new_p >= 0 && new_p < n as i32 && new_q >= 0 && new_q < n as i32 {
                                let new_p = new_p as usize;
                                let new_q = new_q as usize;
                                if grid[new_p][new_q] == 1 && visited[new_p][new_q] == 0 {
                                    queue.push_back((new_p, new_q));
                                }
                            }
                        }
                    }
                    ans = ans.max(c);
                    groups[g] = c;
                }
            }
        }
        if ans == (n * n) as i32 {
            return ans;
        }
        if ans == 0 {
            return 1;
        }
        let mut gs: Vec<usize> = Vec::with_capacity(4);
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut dx = 0;
                    let mut dy = 1;
                    let mut s = 1;
                    for _ in 0..4 {
                        let new_i = i as i32 + dx;
                        let new_j = j as i32 + dy;
                        (dx, dy) = (dy, -dx);
                        if new_i >= 0 && new_i < n as i32 && new_j >= 0 && new_j < n as i32 {
                            let g = visited[new_i as usize][new_j as usize];
                            if !gs.contains(&g) {
                                gs.push(g);
                                s += groups[g];
                            }
                        }
                    }
                    ans = ans.max(s);
                    gs.clear();
                }
            }
        }
        ans
    }
}
