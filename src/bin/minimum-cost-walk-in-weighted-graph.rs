use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{:?}",
        Solution::minimum_cost(
            5,
            vec_vec_i32![[0, 1, 7], [1, 3, 7], [1, 2, 1]],
            vec_vec_i32![[0, 3], [3, 4]],
        )
    );
}

struct Solution;

impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect();
        for e in edges.iter() {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let pa = find(a, &mut parent);
            let pb = find(b, &mut parent);
            if pa != pb {
                parent[pa] = pb;
            }
        }
        let mut cost = vec![-1; n];
        for e in edges.iter() {
            let a = e[0] as usize;
            let pa = find(a, &mut parent);
            cost[pa] &= e[2];
        }
        let mut ans = vec![-1; query.len()];
        for (i, q) in query.iter().enumerate() {
            let a = q[0] as usize;
            let b = q[1] as usize;
            let pa = find(a, &mut parent);
            let pb = find(b, &mut parent);
            if pa == pb {
                ans[i] = cost[pa];
            }
        }
        ans
    }
}

fn find(i: usize, parent: &mut Vec<usize>) -> usize {
    if parent[i] != i {
        parent[i] = find(parent[i], parent);
    }
    parent[i]
}
