fn main() {
    println!(
        "{}",
        Solution::contains_cycle(vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a']
        ])
    );
}

struct Solution;

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] {
                    if dfs(i, j, m, n, m, n, &grid, &mut visited) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn dfs(
    x: usize,
    y: usize,
    p_x: usize,
    p_y: usize,
    m: usize,
    n: usize,
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    let c = grid[x][y];
    visited[x][y] = true;
    let mut dx = 0;
    let mut dy = 1;
    for _ in 0..4 {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        (dx, dy) = (dy, -dx);
        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if new_x < m && new_y < n && (new_x != p_x || new_y != p_y) && grid[new_x][new_y] == c {
                if visited[new_x][new_y] {
                    return true;
                }
                if dfs(new_x, new_y, x, y, m, n, grid, visited) {
                    return true;
                }
            }
        }
    }
    false
}
