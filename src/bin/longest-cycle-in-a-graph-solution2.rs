fn main() {
    println!("{}", Solution::longest_cycle(vec![3, 3, 4, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut visited = vec![0; n];
        let mut parent = vec![n; n];
        let mut ans = -1;
        for i in 0..n {
            if visited[i] == 0 && edges[i] != -1 {
                let mut j = i;
                while edges[j] != -1 && visited[j] == 0 {
                    visited[j] = 1;
                    let v = edges[j] as usize;
                    parent[v] = j;
                    j = v;
                }
                if visited[j] == 1 {
                    let mut size = 0;
                    let mut k = j;
                    while k != n {
                        size += 1;
                        visited[k] = 2;
                        k = parent[k];
                        if k == j {
                            break;
                        }
                    }
                    if k != n {
                        ans = ans.max(size);
                    }
                }
            }
        }
        ans
    }
}
