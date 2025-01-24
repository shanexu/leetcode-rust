fn main() {}

struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn is_safe(i: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<i32>) -> bool {
            if visited[i] > 0 {
                return visited[i] == 2;
            }
            visited[i] = 1;
            for &j in graph[i].iter() {
                if !is_safe(j as usize, graph, visited) {
                    return false;
                }
            }
            visited[i] = 2;
            true
        }
        let n = graph.len();
        let mut visited = vec![0; n];
        let mut ans = vec![];
        for i in 0..n {
            if is_safe(i, &graph, &mut visited) {
                ans.push(i as i32);
            }
        }
        ans
    }
}