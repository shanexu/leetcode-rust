fn main() {
    println!(
        "{}",
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]])
    );
}

struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut parent: Vec<usize> = (0..n).into_iter().collect();
        for i in 0..n {
            for j in i + 1..n {
                if is_connected[i][j] == 1 {
                    let i = find(i, &mut parent);
                    let j = find(j, &mut parent);
                    parent[i] = find(j, &mut parent);
                }
            }
        }
        let mut table: Vec<i32> = vec![0; n];
        for i in 0..n {
            let p = find(i, &mut parent);
            table[p] = 1;
        }
        table.iter().sum()
    }
}

fn find(i: usize, parent: &mut Vec<usize>) -> usize {
    if parent[i] != i {
        parent[i] = find(parent[i], parent);
    }
    parent[i]
}
