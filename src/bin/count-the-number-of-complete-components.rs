use leetcode_rust::vec_vec_i32;

fn main() {
    println!(
        "{}",
        Solution::count_complete_components(6, vec_vec_i32![[0, 1], [0, 2], [1, 2], [3, 4]])
    );
    println!(
        "{}",
        Solution::count_complete_components(
            6,
            vec_vec_i32![[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]]
        )
    );
}

struct Solution;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent = (0..n).into_iter().collect::<Vec<usize>>();
        for e in edges.iter() {
            let a = e[0] as usize;
            let b = e[1] as usize;
            let pa = find(a, &mut parent);
            let pb = find(b, &mut parent);
            parent[pa] = pb;
        }
        let mut vts = vec![0; n];
        for i in 0..n {
            vts[find(i, &mut parent)] += 1;
        }
        let mut eds = vec![0; n];
        for e in edges.iter() {
            eds[find(e[0] as usize, &mut parent)] += 1;
        }
        let mut ans = 0;
        for i in 0..n {
            let vc = vts[i];
            if vc > 0 {
                ans += (eds[i] == vc * (vc - 1) / 2) as i32;
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
