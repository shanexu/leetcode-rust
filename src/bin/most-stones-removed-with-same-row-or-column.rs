fn main() {
    println!(
        "{}",
        Solution::remove_stones(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![2, 2]
        ])
    );
}

struct Solution;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        let mut parent: Vec<usize> = (0..n).into_iter().collect();
        let mut rows: Vec<Vec<usize>> = vec![vec![]; 10001];
        let mut cols: Vec<Vec<usize>> = vec![vec![]; 10001];
        for (i, s) in stones.iter().enumerate() {
            let x = s[0] as usize;
            let y = s[1] as usize;
            for &j in rows[x].iter() {
                let pi = find(i, &mut parent);
                let pj = find(j, &mut parent);
                parent[pi] = pj;
            }
            rows[x].push(i);
            for &j in cols[y].iter() {
                let pi = find(i, &mut parent);
                let pj = find(j, &mut parent);
                parent[pi] = pj;
            }
            cols[y].push(i);
        }
        let mut roots: Vec<i32> = vec![0; n];
        let mut ans = n as i32;
        for i in 0..n {
            let p = find(i, &mut parent);
            ans -= roots[p] ^ 1;
            roots[p] = 1;
        }
        ans
    }
}

fn find(x: usize, parent: &mut Vec<usize>) -> usize {
    if parent[x] != x {
        parent[x] = find(parent[x], parent);
    }
    parent[x]
}
