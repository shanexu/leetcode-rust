fn main() {
    println!(
        "{:?}",
        Solution::check_if_prerequisite(
            5,
            vec![
                vec![4, 3],
                vec![4, 1],
                vec![4, 0],
                vec![3, 2],
                vec![3, 1],
                vec![3, 0],
                vec![2, 1],
                vec![2, 0],
                vec![1, 0]
            ],
            vec![vec![4, 2]]
        )
    )
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = num_courses as usize;
        let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
        let mut in_degrees: Vec<usize> = vec![0; n];
        let mut ans = Vec::with_capacity(queries.len());
        for p in prerequisites {
            let from = p[0] as usize;
            let to = p[1] as usize;
            edges[from].push(to);
            in_degrees[to] += 1;
        }
        let mut queue = VecDeque::new();
        let mut reachable = vec![vec![false; n]; n];
        for i in 0..n {
            if in_degrees[i] == 0 {
                queue.push_back(i);
            }
        }
        while let Some(u) = queue.pop_front() {
            for &v in edges[u].iter() {
                reachable[u][v] = true;
                for k in 0..n {
                    reachable[k][v] |= reachable[k][u]
                }
                in_degrees[v] -= 1;
                if in_degrees[v] == 0 {
                    queue.push_back(v);
                }
            }
        }
        for q in queries.iter() {
            ans.push(reachable[q[0] as usize][q[1] as usize]);
        }
        ans
    }
}
