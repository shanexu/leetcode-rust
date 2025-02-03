fn main() {
    println!(
        "{}",
        Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2]
        ])
    );
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut visited = vec![false; n];
        let mut rows: Vec<Vec<usize>> = vec![vec![]; 10001];
        let mut cols: Vec<Vec<usize>> = vec![vec![]; 10001];
        for (i, s) in stones.iter().enumerate() {
            let x = s[0] as usize;
            let y = s[1] as usize;
            rows[x].push(i);
            cols[y].push(i);
        }
        let mut queue = VecDeque::new();
        let mut ans = n as i32;
        for i in 0..n {
            if !visited[i] {
                ans -= 1;
                queue.push_back(i);
                while let Some(u) = queue.pop_front() {
                    if !visited[u] {
                        visited[u] = true;
                        let x = stones[u][0] as usize;
                        let y = stones[u][1] as usize;
                        for &v in rows[x].iter() {
                            if v != u && !visited[v] {
                                queue.push_back(v);
                            }
                        }
                        for &v in cols[y].iter() {
                            if v != u && !visited[v] {
                                queue.push_back(v);
                            }
                        }
                    }
                }
            }
        }
        ans
    }
}
