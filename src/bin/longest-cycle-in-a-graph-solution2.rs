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
        for mut i in 0..n {
            while edges[i] != -1 && visited[i] == 0 {
                visited[i] = 1;
                let v = edges[i] as usize;
                parent[v] = i;
                i = v;
            }
            if visited[i] == 1 {
                let mut size = 0;
                let mut k = i;
                while k != n {
                    size += 1;
                    visited[k] = 2;
                    k = parent[k];
                    if k == i {
                        break;
                    }
                }
                if k != n {
                    ans = ans.max(size);
                }
            }
        }
        ans
    }
}
