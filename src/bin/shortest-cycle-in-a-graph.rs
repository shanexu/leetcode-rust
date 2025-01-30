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
        let mut ans = i32::MAX;
        let mut parent: Vec<usize> = vec![n; n];
        let mut dist: Vec<i32> = vec![MAX; n];
        let mut queue = VecDeque::new();
        for i in 0..n {
            dist.fill(MAX);
            parent.fill(n);
            dist[i] = 0;
            queue.push_back(i);
            while let Some(x) = queue.pop_front() {
                for &c in adj[x].iter() {
                    if dist[c] == MAX {
                        dist[c] = 1 + dist[x];
                        parent[c] = x;
                        queue.push_back(c);
                    } else if parent[x] != c && parent[c] != x {
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

const MAX: i32 = 1000000000;
