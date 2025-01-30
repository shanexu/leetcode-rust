fn main() {
    println!(
        "{}",
        Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]])
    );
    println!(
        "{}",
        Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]])
    );
}

struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colors = vec![0; n];
        for i in 0..n {
            if colors[i] == 0 && !dfs(i, &graph, &mut colors, 1) {
                return false;
            }
        }
        true
    }
}

fn dfs(u: usize, graph: &Vec<Vec<i32>>, colors: &mut Vec<u8>, color: u8) -> bool {
    colors[u] = color;
    for &v in graph[u].iter() {
        let v = v as usize;
        if colors[v] == 0 {
            if !dfs(v, graph, colors, 3 - color) {
                return false;
            }
        } else if colors[v] == color {
            return false;
        }
    }
    true
}
