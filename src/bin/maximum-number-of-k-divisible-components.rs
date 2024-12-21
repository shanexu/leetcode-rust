fn main() {
    Solution::max_k_divisible_components(
        7,
        vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
        ],
        vec![
            1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000,
        ],
        7,
    );

    Solution::max_k_divisible_components(
        3,
        vec![
            vec![0, 1],
            vec![0, 2],
        ],
        vec![
            1000000000, 1000000000, 1000000000
        ],
        3,
    );
}

struct Solution;

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        mut values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }
        let mut count = 0;
        let mut edge_maps = vec![vec![]; n];
        for e in edges {
            let from = e[0] as usize;
            let to = e[1] as usize;
            edge_maps[from].push(to);
            edge_maps[to].push(from);
        }
        let mut parents = vec![n; n];
        let mut stack1 = vec![0];
        let mut stack2 = vec![];
        while let Some(node) = stack1.pop() {
            stack2.push(node);
            let parent = parents[node];
            for &child in edge_maps[node].iter() {
                if child != parent {
                    parents[child] = node;
                    stack1.push(child);
                }
            }
        }
        while let Some(node) = stack2.pop() {
            let v = values[node];
            let r = v % k;
            if r == 0 {
                count += 1;
            } else {
                values[parents[node]] += r; // 因为会出现溢出
            }
        }
        count
    }
}
