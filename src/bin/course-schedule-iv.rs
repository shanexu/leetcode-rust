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
        let mut ans = Vec::with_capacity(queries.len());
        for p in prerequisites {
            edges[p[0] as usize].push(p[1] as usize);
        }
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        let mut reachable = vec![vec![false; n]; n];
        for i in 0..n {
            visited.fill(false);
            queue.push_back(vec![i]);
            while let Some(u) = queue.pop_front() {
                if visited[u[u.len() - 1]] {
                    continue;
                }
                visited[u[u.len() - 1]] = true;
                for &v in &edges[u[u.len() - 1]] {
                    for &k in u.iter() {
                        reachable[k][v] = true;
                    }
                    let mut u = u.clone();
                    u.push(v);
                    queue.push_back(u);
                }
            }
        }
        for q in queries.iter() {
            ans.push(reachable[q[0] as usize][q[1] as usize]);
        }
        ans
    }
}
