use leetcode_rust::vec_vec_i32;

fn main() {
    assert_eq!(
        6,
        Solution::most_profitable_path(
            vec_vec_i32![[0, 1], [1, 2], [1, 3], [3, 4]],
            3,
            vec![-2, 4, 2, -4, 6]
        )
    );
}

struct Solution;

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let bob = bob as usize;
        let n = amount.len();
        let mut graph = vec![vec![]; n];
        for e in &edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut bob_steps = vec![n; n];
        bob_steps[bob] = 0;
        dfs_bob(&graph, bob, n, &mut bob_steps, 0);
        let mut ans = i32::MIN;
        dfs_alice(&graph, &amount, 0, n, &bob_steps, 0, 0, &mut ans);
        ans
    }
}

fn dfs_bob(
    graph: &Vec<Vec<usize>>,
    current: usize,
    prev: usize,
    steps: &mut Vec<usize>,
    step: usize,
) -> bool {
    if current == 0 {
        steps[0] = steps[0].min(step);
        return true;
    }
    for &next in graph[current].iter() {
        if next != prev && dfs_bob(graph, next, current, steps, step + 1) {
            steps[next] = steps[next].min(step + 1);
            return true;
        }
    }
    false
}

fn dfs_alice(
    graph: &Vec<Vec<usize>>,
    amount: &Vec<i32>,
    current: usize,
    prev: usize,
    steps: &Vec<usize>,
    step: usize,
    mut profit: i32,
    ans: &mut i32,
) {
    if step == steps[current] {
        profit += amount[current] / 2;
    } else if step < steps[current] {
        profit += amount[current];
    }
    if graph[current].len() == 1 && graph[current][0] == prev {
        *ans = (*ans).max(profit);
    }
    for &next in graph[current].iter() {
        if next != prev {
            dfs_alice(graph, amount, next, current, steps, step + 1, profit, ans);
        }
    }
}
