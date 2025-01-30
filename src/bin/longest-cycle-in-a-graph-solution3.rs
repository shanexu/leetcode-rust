fn main() {
    println!("{}", Solution::longest_cycle(vec![3, 3, 4, 2, 3]));
}

struct Solution;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let n = edges.len();
        let mut visited = vec![0; n];
        let mut dist = vec![0; n];
        let mut ans = -1;
        for mut i in 0..n {
            let m = i + 1;
            let mut d = 1;
            while edges[i] != -1 && visited[i] == 0 {
                visited[i] = m;
                dist[i] = d;
                i = edges[i] as usize;
                d += 1;
            }
            if visited[i] == m {
                ans = ans.max(d - dist[i]);
            }
        }
        ans
    }
}
