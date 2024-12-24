fn main() {
    println!(
        "{}",
        Solution::minimum_diameter_after_merge(
            vec![
                vec![0, 1],
                vec![2, 0],
                vec![3, 2],
                vec![3, 6],
                vec![8, 7],
                vec![4, 8],
                vec![5, 4],
                vec![3, 5],
                vec![3, 9]
            ],
            vec![vec![0, 1], vec![0, 2], vec![0, 3]]
        )
    );
    println!(
        "{}",
        Solution::minimum_diameter_after_merge(
            vec![vec![0, 1], vec![0, 2], vec![0, 3],],
            vec![vec![0, 1]]
        )
    );
}

struct Solution;

impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let g1 = edges_to_graph(&edges1);
        let g2 = edges_to_graph(&edges2);
        let d1 = bfs(bfs(0, &g1).0, &g1).1;
        let d2 = bfs(bfs(0, &g2).0, &g2).1;
        let mut m = div2(d1) + div2(d2) + 1;
        m = std::cmp::max(m, d1);
        m = std::cmp::max(m, d2);
        m as i32
    }
}

#[inline]
fn edges_to_graph(edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; edges.len() + 1];
    for e in edges.iter() {
        let u = e[0] as usize;
        let v = e[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }
    graph
}

#[inline]
fn bfs(u: usize, graph: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut dis: Vec<i32> = vec![-1; graph.len()];
    let mut stack: Vec<usize> = vec![u];
    dis[u] = 0;
    let mut max_dis = 0;
    let mut idx = 0;
    while !stack.is_empty() {
        let t = stack.pop().unwrap();
        for &x in &graph[t] {
            if dis[x] == -1 {
                stack.push(x);
                dis[x] = dis[t] + 1;
                if dis[x] > max_dis {
                    max_dis = dis[x];
                    idx = x;
                }
            }
        }
    }
    (idx, max_dis as usize)
}
#[inline]
fn div2(x: usize) -> usize {
    (x + 1) >> 1
}