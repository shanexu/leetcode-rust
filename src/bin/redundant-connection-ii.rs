fn main() {
    println!(
        "{:?}",
        Solution::find_redundant_directed_connection(vec![
            vec![4, 2],
            vec![1, 5],
            vec![5, 2],
            vec![5, 3],
            vec![2, 4]
        ])
    );
    println!(
        "{:?}",
        Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]])
    );
}

/// 有几种情况：
/// 1. 有入度为2的节点，有环：环中指向该节点的边
/// 2. 有入度为2的节点，无环：任意指向该节点的边
/// 3. 无入度为2的节点，有环：环中任意边
struct Solution;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut adj = vec![vec![]; n + 1];
        let mut in_degrees = vec![0; n + 1];
        for edge in edges.iter() {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            adj[n1].push(n2);
            in_degrees[n2] += 1;
        }
        let mut node_having_two_parents = 0;
        for i in 1..=n {
            if in_degrees[i] == 2 {
                node_having_two_parents = i;
                break;
            }
        }
        let mut visited = vec![false; n + 1];
        let mut rec_stack = vec![false; n + 1];
        let mut visited_nodes = vec![];
        for i in 1..=n {
            if !visited[n] {
                if let Some(cycle) = dfs(i, &adj, &mut visited, &mut rec_stack, &mut visited_nodes)
                {
                    for edge in edges.iter().rev() {
                        let n1 = edge[0] as usize;
                        let n2 = edge[1] as usize;
                        if node_having_two_parents != 0 {
                            if n2 == node_having_two_parents && cycle[n1] {
                                return vec![n1 as i32, n2 as i32];
                            }
                        } else if cycle[n1] && cycle[n2] {
                            return vec![n1 as i32, n2 as i32];
                        }
                    }
                }
            }
        }
        for edge in edges.iter().rev() {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            if n2 == node_having_two_parents {
                return vec![n1 as i32, n2 as i32];
            }
        }
        vec![0, 0]
    }
}

fn dfs(
    u: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    rec_stack: &mut Vec<bool>,
    visited_nodes: &mut Vec<usize>,
) -> Option<Vec<bool>> {
    visited[u] = true;
    rec_stack[u] = true;
    visited_nodes.push(u);
    for &v in adj[u].iter() {
        if !visited[v] {
            let result = dfs(v, &adj, visited, rec_stack, visited_nodes);
            if result.is_some() {
                return result;
            }
        } else if rec_stack[v] {
            let mut cycle = vec![false; adj.len()];
            for &j in visited_nodes.iter().rev() {
                cycle[j] = true;
                if j == v {
                    break;
                }
            }
            return Some(cycle);
        }
    }
    rec_stack[u] = false;
    visited_nodes.pop();
    None
}
