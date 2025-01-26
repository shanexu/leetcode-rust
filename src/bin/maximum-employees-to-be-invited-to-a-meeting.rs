fn main() {
    println!("{}", Solution::maximum_invitations(vec![3, 0, 1, 4, 1]));
    println!("{}", Solution::maximum_invitations(vec![2, 2, 1, 2]));
    println!("{}", Solution::maximum_invitations(vec![2, 2, 1, 2, 1]));
}

struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        fn find_max_cycle(favorite: &Vec<i32>) -> i32 {
            let n = favorite.len();
            let mut visited = vec![false; n];
            let mut max_cycle = 0;
            for i in 0..n {
                if visited[i] {
                    continue;
                }
                let mut visited_nodes = vec![];
                let mut current_node = i;
                while !visited[current_node] {
                    visited_nodes.push(current_node);
                    visited[current_node] = true;
                    current_node = favorite[current_node] as usize;
                }
                let m = visited_nodes.len();
                for (k, &node) in visited_nodes.iter().enumerate() {
                    if node == current_node {
                        max_cycle = max_cycle.max(m - k);
                        break;
                    }
                }
            }
            max_cycle as i32
        }
        fn topological_sort(favorite: &Vec<i32>) -> i32 {
            let n = favorite.len();
            let mut in_degree = vec![0; n];
            let mut distance = vec![1; n];
            for &f in favorite.iter() {
                in_degree[f as usize] += 1;
            }
            let mut queue = VecDeque::new();
            for (i, &d) in in_degree.iter().enumerate() {
                if d == 0 {
                    queue.push_back(i);
                }
            }
            while let Some(current_node) = queue.pop_front() {
                let next_node = favorite[current_node] as usize;
                distance[next_node] = distance[next_node].max(distance[current_node] + 1);
                if in_degree[next_node] > 0 {
                    in_degree[next_node] -= 1;
                    if in_degree[next_node] == 0 {
                        queue.push_back(next_node);
                    }
                }
            }
            let mut ans = 0;
            for i in 0..n {
                if i == favorite[favorite[i] as usize] as usize {
                    ans += distance[i];
                }
            }
            ans
        }
        let mut ans = 0;
        ans = ans.max(find_max_cycle(&favorite));
        ans = ans.max(topological_sort(&favorite));
        ans
    }
}
