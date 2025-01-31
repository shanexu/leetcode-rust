fn main() {
    println!("{}", Solution::largest_island(vec![vec![1, 0], vec![0, 1]]));
}

struct Solution;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if n == 1 {
            return 1;
        }
        let mut ans = 0;
        let mut parent = vec![0; n * n];
        let mut cluster_size = vec![0; n * n];
        for i in 0..n * n {
            parent[i] = i;
            cluster_size[i] = 1;
        }
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let mut dx = 0;
                    let mut dy = 1;
                    for _ in 0..4 {
                        let x = i as i32 + dx;
                        let y = j as i32 + dy;
                        (dx, dy) = (dy, -dx);
                        if x >= 0 && x < n as i32 && y >= 0 && y < n as i32 {
                            let x = x as usize;
                            let y = y as usize;
                            if grid[x][y] == 1 {
                                let root_neighbor = find(x * n + y, &mut parent);
                                let root_current = find(i * n + j, &mut parent);
                                if root_neighbor != root_current {
                                    parent[root_neighbor] = root_current;
                                    cluster_size[root_current] += cluster_size[root_neighbor];
                                    ans = ans.max(cluster_size[root_current]);
                                }
                            }
                        }
                    }
                }
            }
        }
        if ans == (n * n) as i32 {
            return ans;
        }
        let mut visited = Vec::with_capacity(4);
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 0 {
                    let mut s = 1;
                    let mut dx = 0;
                    let mut dy = 1;
                    for _ in 0..4 {
                        let x = i as i32 + dx;
                        let y = j as i32 + dy;
                        (dx, dy) = (dy, -dx);
                        if x >= 0 && x < n as i32 && y >= 0 && y < n as i32 {
                            let x = x as usize;
                            let y = y as usize;
                            if grid[x][y] == 1 {
                                let root = find(x * n + y, &mut parent);
                                if !visited.contains(&root) {
                                    visited.push(root);
                                    s += cluster_size[root];
                                }
                            }
                        }
                    }
                    visited.clear();
                    ans = ans.max(s);
                }
            }
        }
        ans
    }
}

#[inline(always)]
fn find(index: usize, parent: &mut Vec<usize>) -> usize {
    if parent[index] != index {
        parent[index] = find(parent[index], parent);
    }
    return parent[index];
}
