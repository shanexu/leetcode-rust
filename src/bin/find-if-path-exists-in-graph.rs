fn main() {
    println!(
        "{}",
        Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2)
    );
}

struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut visited = vec![false; n];
        fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, src: usize, dest: usize) -> bool {
            if src == dest {
                return true;
            }
            visited[src] = true;
            for &v in graph[src].iter() {
                if !visited[v] && dfs(graph, visited, v, dest) {
                    return true;
                }
            }
            return false;
        }
        dfs(&graph, &mut visited, source as usize, destination as usize)
    }
}
