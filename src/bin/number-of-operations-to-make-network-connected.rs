fn main() {
    println!(
        "{}",
        Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]])
    );
    println!(
        "{}",
        Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]])
    );
}

struct Solution;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        if connections.len() < n - 1 {
            return -1;
        }
        let mut parent: Vec<usize> = (0..n).into_iter().collect();
        for e in connections.iter() {
            let n1 = e[0] as usize;
            let n2 = e[1] as usize;
            let p1 = find(n1, &mut parent);
            let p2 = find(n2, &mut parent);
            parent[p1] = p2;
        }
        let mut roots: Vec<i32> = vec![0; n];
        for i in 0..n {
            roots[find(i, &mut parent)] = 1;
        }
        roots.iter().sum::<i32>() - 1
    }
}

#[inline]
fn find(i: usize, parent: &mut Vec<usize>) -> usize {
    if i != parent[i] {
        parent[i] = find(parent[i], parent)
    }
    parent[i]
}
