fn main() {
    println!("{}", Solution::longest_cycle(vec![3, 3, 4, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut visited = vec![false; n];
        let mut rec_stack = vec![false; n];
        let mut parent = vec![n; n];
        let mut ans = -1;
        for i in 0..n {
            if visited[i] {
                continue;
            }
            dfs(
                i,
                &edges,
                &mut visited,
                &mut rec_stack,
                &mut parent,
                &mut ans,
            );
        }
        ans
    }
}

fn dfs(
    u: usize,
    edges: &Vec<i32>,
    visited: &mut Vec<bool>,
    rec_stack: &mut Vec<bool>,
    parent: &mut Vec<usize>,
    ans: &mut i32,
) {
    visited[u] = true;
    rec_stack[u] = true;
    if edges[u] != -1 {
        let v = edges[u] as usize;
        if !visited[v] {
            parent[v] = u;
            dfs(v, edges, visited, rec_stack, parent, ans);
        } else if rec_stack[v] {
            let mut cur = u;
            let mut size = 1;
            while cur != v {
                size += 1;
                cur = parent[cur];
            }
            *ans = (*ans).max(size);
        }
    }
    rec_stack[u] = false;
}
