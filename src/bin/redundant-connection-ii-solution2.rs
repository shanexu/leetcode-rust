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
        let mut parents: Vec<usize> = (0..=n).into_iter().collect();
        let mut parents2 = parents.clone();
        let mut conflict_edge_idx = n + 1;
        let mut cycle_edge_idx = n + 1;
        for (i, edge) in edges.iter().enumerate() {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            if parents[to] != to {
                conflict_edge_idx = i;
                if cycle_edge_idx != n + 1 {
                    break;
                }
            } else {
                parents[to] = from;
                if !union(from, to, &mut parents2) {
                    cycle_edge_idx = i;
                    if conflict_edge_idx != n + 1 {
                        break;
                    }
                }
            }
        }
        if conflict_edge_idx == n + 1 {
            return edges[cycle_edge_idx].clone();
        }
        let to = edges[conflict_edge_idx][1] as usize;
        if cycle_edge_idx != n + 1 {
            return vec![parents[to] as i32, to as i32];
        }
        return edges[conflict_edge_idx].clone();
    }
}

#[inline(always)]
fn union(a: usize, b: usize, parents: &mut Vec<usize>) -> bool {
    let pa = find(a, parents);
    let pb = find(b, parents);
    if pa == pb {
        return false;
    }
    parents[pa] = pb;
    return true;
}

#[inline(always)]
fn find(x: usize, parents: &mut Vec<usize>) -> usize {
    if parents[x] != x {
        parents[x] = find(parents[x], parents);
    }
    parents[x]
}
