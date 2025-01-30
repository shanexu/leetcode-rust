fn main() {
    println!(
        "{}",
        Solution::magnificent_sets(
            92,
            vec![
                vec![67, 29],
                vec![13, 29],
                vec![77, 29],
                vec![36, 29],
                vec![82, 29],
                vec![54, 29],
                vec![57, 29],
                vec![53, 29],
                vec![68, 29],
                vec![26, 29],
                vec![21, 29],
                vec![46, 29],
                vec![41, 29],
                vec![45, 29],
                vec![56, 29],
                vec![88, 29],
                vec![2, 29],
                vec![7, 29],
                vec![5, 29],
                vec![16, 29],
                vec![37, 29],
                vec![50, 29],
                vec![79, 29],
                vec![91, 29],
                vec![48, 29],
                vec![87, 29],
                vec![25, 29],
                vec![80, 29],
                vec![71, 29],
                vec![9, 29],
                vec![78, 29],
                vec![33, 29],
                vec![4, 29],
                vec![44, 29],
                vec![72, 29],
                vec![65, 29],
                vec![61, 29]
            ],
        )
    );
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn magnificent_sets(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n + 1];
        for e in edges.iter() {
            let a = e[0] as usize;
            let b = e[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut visited = vec![false; n + 1];
        let mut visited_nodes = Vec::with_capacity(n);
        let mut depth = vec![0; n + 1];
        let mut queue = VecDeque::with_capacity(n);
        let mut ans = 0;
        for i in 1..=n {
            if visited[i] {
                continue;
            }
            let mut max_depth = -1;
            dfs(i, &adj, &mut visited, &mut visited_nodes);
            for &u in visited_nodes.iter() {
                max_depth = max_depth.max(bfs(u, &adj, &visited_nodes, &mut depth, &mut queue));
            }
            if max_depth == -1 {
                return -1;
            }
            ans += max_depth;
            visited_nodes.clear();
        }
        ans
    }
}

#[inline(always)]
fn dfs(u: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, visited_nodes: &mut Vec<usize>) {
    visited_nodes.push(u);
    visited[u] = true;
    for &v in adj[u].iter() {
        if !visited[v] {
            dfs(v, adj, visited, visited_nodes);
        }
    }
}

#[inline(always)]
fn bfs(
    u: usize,
    adj: &Vec<Vec<usize>>,
    visited_nodes: &Vec<usize>,
    depth: &mut Vec<i32>,
    queue: &mut VecDeque<usize>,
) -> i32 {
    depth.fill(i32::MAX);
    depth[u] = 1;
    let mut max_depth = 1;
    queue.push_back(u);
    while let Some(u) = queue.pop_front() {
        for &v in adj[u].iter() {
            if depth[v] == i32::MAX {
                depth[v] = depth[u] + 1;
                max_depth = depth[v];
                queue.push_back(v);
            }
        }
    }

    for &u in visited_nodes {
        for &v in adj[u].iter() {
            if (depth[u] - depth[v]).abs() != 1 {
                return -1;
            }
        }
    }
    max_depth
}
