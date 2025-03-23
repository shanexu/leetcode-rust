use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{}",
        Solution::count_paths(
            12,
            vec_vec_i32![
                [1, 0, 2348],
                [2, 1, 2852],
                [2, 0, 5200],
                [3, 1, 12480],
                [2, 3, 9628],
                [4, 3, 7367],
                [4, 0, 22195],
                [5, 4, 5668],
                [1, 5, 25515],
                [0, 5, 27863],
                [6, 5, 836],
                [6, 0, 28699],
                [2, 6, 23499],
                [6, 3, 13871],
                [1, 6, 26351],
                [5, 7, 6229],
                [2, 7, 28892],
                [1, 7, 31744],
                [3, 7, 19264],
                [6, 7, 5393],
                [2, 8, 31998],
                [8, 7, 3106],
                [3, 8, 22370],
                [8, 4, 15003],
                [8, 6, 8499],
                [8, 5, 9335],
                [8, 9, 5258],
                [9, 2, 37256],
                [3, 9, 27628],
                [7, 9, 8364],
                [1, 9, 40108],
                [9, 5, 14593],
                [2, 10, 45922],
                [5, 10, 23259],
                [9, 10, 8666],
                [10, 0, 51122],
                [10, 3, 36294],
                [10, 4, 28927],
                [11, 4, 25190],
                [11, 9, 4929],
                [11, 8, 10187],
                [11, 6, 18686],
                [2, 11, 42185],
                [11, 3, 32557],
                [1, 11, 45037]
            ],
        )
    );
}

struct Solution;

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for e in roads {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let d = e[2];
            graph[a].push((b, d));
            graph[b].push((a, d));
        }
        let mut dist = vec![i64::MAX; n];
        dist[0] = 0;
        let mut count = vec![0; n];
        count[0] = 1;
        let mut queue = BinaryHeap::new();
        queue.push((Reverse(0), 0));
        while let Some((Reverse(d), u)) = queue.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in graph[u].iter() {
                let alt = w as i64 + d;
                if alt < dist[v] {
                    dist[v] = alt;
                    count[v] = count[u];
                    queue.push((Reverse(alt), v));
                } else if alt == dist[v] {
                    count[v] = (count[v] + count[u]) % MOD;
                }
            }
        }
        count[n - 1]
    }
}

const MOD: i32 = 1_000_000_007;
