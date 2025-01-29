fn main() {
    println!(
        "{:?}",
        Solution::find_redundant_connection(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![1, 4],
            vec![1, 5]
        ])
    );
}

struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
        for edge in edges.iter() {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut visited = vec![false; n + 1];
        let mut visited_nodes: Vec<usize> = vec![0];
        fn dfs(
            v: usize,
            adj: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            visited_nodes: &mut Vec<usize>,
        ) -> Option<Vec<bool>> {
            visited[v] = true;
            let prev = visited_nodes[visited_nodes.len() - 1];
            visited_nodes.push(v);
            for &u in adj[v].iter() {
                if !visited[u] {
                    let result = dfs(u, &adj, visited, visited_nodes);
                    if result.is_some() {
                        return result;
                    }
                } else if u != prev {
                    let mut cycle = vec![false; adj.len()];
                    for &j in visited_nodes.iter().rev() {
                        cycle[j] = true;
                        if j == u {
                            break;
                        }
                    }
                    return Some(cycle);
                }
            }
            visited_nodes.pop();
            return None;
        }
        for i in 1..=n {
            if !visited[i] {
                if let Some(cycle) = dfs(i, &adj, &mut visited, &mut visited_nodes) {
                    for edge in edges.iter().rev() {
                        if cycle[edge[0] as usize] && cycle[edge[1] as usize] {
                            return edge.clone();
                        }
                    }
                }
            }
        }
        vec![0, 0]
    }
}
