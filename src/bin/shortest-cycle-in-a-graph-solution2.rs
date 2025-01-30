fn main() {
    println!(
        "{}",
        Solution::find_shortest_cycle(
            8,
            vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![0, 7],
                vec![0, 6],
                vec![5, 7],
                vec![5, 6],
            ],
        )
    );
}

/// O(V*E)
struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for edge in edges.iter() {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut par: Vec<usize> = vec![n; n];
        let mut on_circle: Vec<bool> = vec![false; n];
        let mut color: Vec<u8> = vec![0; n];
        for i in 0..n {
            dfs(i, n, &adj, &mut color, &mut par, &mut on_circle);
        }
        let mut ans = i32::MAX;
        let mut dist: Vec<i32> = vec![MAX; n];
        let mut queue = VecDeque::new();
        for i in 0..n {
            if !on_circle[i] {
                continue;
            }
            dist.fill(MAX);
            par.fill(n);
            dist[i] = 0;
            queue.push_back(i);
            while let Some(x) = queue.pop_front() {
                for &c in adj[x].iter() {
                    if dist[c] == MAX {
                        dist[c] = 1 + dist[x];
                        par[c] = x;
                        queue.push_back(c);
                    } else if par[x] != c && par[c] != x {
                        ans = ans.min(dist[x] + dist[c] + 1);
                    }
                }
            }
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}

fn dfs(
    u: usize,
    p: usize,
    adj: &Vec<Vec<usize>>,
    color: &mut Vec<u8>,
    par: &mut Vec<usize>,
    on_circle: &mut Vec<bool>,
) {
    if color[u] == 2 {
        return;
    }
    if color[u] == 1 {
        let mut cur = p;
        on_circle[cur] = true;
        while cur != u {
            cur = par[cur];
            on_circle[cur] = true;
        }
        return;
    }
    par[u] = p;
    color[u] = 1;
    for &v in adj[u].iter() {
        if v == par[u] {
            continue;
        }
        dfs(v, u, adj, color, par, on_circle);
    }
    color[u] = 2;
}

const MAX: i32 = 1000000000;
