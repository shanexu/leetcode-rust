fn main() {
    println!(
        "{}",
        Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![0, 2]], 0, 2)
    );
}

struct Solution;

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).into_iter().collect();
        for e in edges.iter() {
            let a = find(e[0] as usize, &mut parent);
            let b = find(e[1] as usize, &mut parent);
            parent[a] = find(b, &mut parent);
        }
        find(source as usize, &mut parent) == find(destination as usize, &mut parent)
    }
}

fn find(i: usize, parent: &mut Vec<usize>) -> usize {
    if parent[i] != i {
        parent[i] = find(parent[i], parent);
    }
    parent[i]
}
