fn main() {
    // println!(
    //     "{:?}",
    //     Solution::find_redundant_directed_connection(vec![
    //         vec![4, 2],
    //         vec![1, 5],
    //         vec![5, 2],
    //         vec![5, 3],
    //         vec![2, 4]
    //     ])
    // );
    println!(
        "{:?}",
        Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]])
    );
}

struct Solution;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut adj = vec![vec![]; n + 1];
        let mut in_degrees = vec![0; n + 1];
        let mut out_degrees = vec![0; n + 1];
        for edge in edges.iter() {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            adj[n1].push(n2);
            in_degrees[n2] += 1;
            out_degrees[n1] += 1;
        }
        let mut two_in_degree = 0;
        for i in 1..=n {
            if in_degrees[i] == 2 {
                two_in_degree = i;
                break;
            }
        }
        let mut visited = vec![false; n + 1];
        let mut rec_stack = vec![false; n + 1];
        let mut visited_nodes = vec![];
        fn dfs(
            u: usize,
            adj: &Vec<Vec<usize>>,
            visited: &mut Vec<bool>,
            rec_stack: &mut Vec<bool>,
            visited_nodes: &mut Vec<usize>,
        ) -> Option<Vec<usize>> {
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
                    let mut cycle = vec![];
                    for &j in visited_nodes.iter().rev() {
                        cycle.push(j);
                        if j == v {
                            break;
                        }
                    }
                    cycle.reverse();
                    return Some(cycle);
                }
            }
            rec_stack[u] = false;
            visited_nodes.pop();
            None
        }
        for i in 1..=n {
            if !visited[n] {
                if let Some(cycle) = dfs(i, &adj, &mut visited, &mut rec_stack, &mut visited_nodes)
                {
                    let mut indices = vec![n + 1; n + 1];
                    let cycle_size = cycle.len();
                    for k in 0..cycle_size {
                        indices[cycle[k]] = k;
                    }
                    for edge in edges.iter().rev() {
                        let n1 = edge[0] as usize;
                        let n2 = edge[1] as usize;
                        if two_in_degree != 0 {
                            if n2 == two_in_degree && indices[n1] != n + 1 {
                                return vec![n1 as i32, n2 as i32];
                            }
                        } else if indices[n1] != n + 1
                            && cycle[(indices[n1] + 1) % cycle_size] == n2
                        {
                            return vec![n1 as i32, n2 as i32];
                        }
                    }
                }
            }
        }
        for edge in edges.iter().rev() {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            if n2 == two_in_degree {
                return vec![n1 as i32, n2 as i32];
            }
        }
        vec![0, 0]
    }
}
