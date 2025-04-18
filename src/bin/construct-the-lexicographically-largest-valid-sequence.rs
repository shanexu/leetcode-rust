fn main() {
    for i in 1..=20 {
        assert_eq!(
            construct_distanced_sequence(i),
            Solution::construct_distanced_sequence(i)
        );
    }
}

struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        fn dfs(index: usize, n: usize, path: &mut Vec<i32>, count: &mut Vec<bool>) -> bool {
            if index == path.len() {
                return true;
            }
            if path[index] > 0 {
                return dfs(index + 1, n, path, count);
            }
            for i in (2..=n).rev() {
                if count[i] && index + i < path.len() && path[index + i] == 0 {
                    count[i] = false;
                    path[index] = i as i32;
                    path[index + i] = i as i32;

                    if dfs(index + 1, n, path, count) {
                        return true;
                    }

                    count[i] = true;
                    path[index] = 0;
                    path[index + i] = 0;
                }
            }
            if count[1] {
                path[index] = 1;
                count[1] = false;
                if dfs(index + 1, n, path, count) {
                    return true;
                }
                count[1] = true;
                path[index] = 0;
            }
            return false;
        }
        let n = n as usize;
        let mut path = vec![0; n * 2 - 1];
        let mut count = vec![true; n + 1];
        if dfs(0, n, &mut path, &mut count) {
            return path;
        }
        vec![]
    }
}

#[rustfmt::skip]
fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    match n {
        1 => vec![1],
        2 => vec![2, 1, 2],
        3 => vec![3, 1, 2, 3, 2],
        4 => vec![4, 2, 3, 2, 4, 3, 1],
        5 => vec![5, 3, 1, 4, 3, 5, 2, 4, 2],
        6 => vec![6, 4, 2, 5, 2, 4, 6, 3, 5, 1, 3],
        7 => vec![7, 5, 3, 6, 4, 3, 5, 7, 4, 6, 2, 1, 2],
        8 => vec![8, 6, 4, 2, 7, 2, 4, 6, 8, 5, 3, 7, 1, 3, 5],
        9 => vec![9, 7, 5, 3, 8, 6, 3, 5, 7, 9, 4, 6, 8, 2, 4, 2, 1],
        10 => vec![10, 8, 6, 9, 3, 1, 7, 3, 6, 8, 10, 5, 9, 7, 4, 2, 5, 2, 4],
        11 => vec![11, 9, 10, 6, 4, 1, 7, 8, 4, 6, 9, 11, 10, 7, 5, 8, 2, 3, 2, 5, 3],
        12 => vec![12, 10, 11, 7, 5, 3, 8, 9, 3, 5, 7, 10, 12, 11, 8, 6, 9, 2, 4, 2, 1, 6, 4],
        13 => vec![13, 11, 12, 8, 6, 4, 9, 10, 1, 4, 6, 8, 11, 13, 12, 9, 7, 10, 3, 5, 2, 3, 2, 7, 5],
        14 => vec![14, 12, 13, 9, 7, 11, 4, 1, 10, 8, 4, 7, 9, 12, 14, 13, 11, 8, 10, 6, 3, 5, 2, 3, 2, 6, 5],
        15 => vec![15, 13, 14, 10, 8, 12, 5, 3, 11, 9, 3, 5, 8, 10, 13, 15, 14, 12, 9, 11, 7, 4, 6, 1, 2, 4, 2, 7, 6],
        16 => vec![16, 14, 15, 11, 9, 13, 6, 4, 12, 10, 1, 4, 6, 9, 11, 14, 16, 15, 13, 10, 12, 8, 5, 7, 2, 3, 2, 5, 3, 8, 7],
        17 => vec![17, 15, 16, 12, 10, 14, 7, 5, 3, 13, 11, 3, 5, 7, 10, 12, 15, 17, 16, 14, 9, 11, 13, 8, 6, 2, 1, 2, 4, 9, 6, 8, 4],
        18 => vec![18, 16, 17, 13, 11, 15, 8, 14, 4, 2, 12, 2, 4, 10, 8, 11, 13, 16, 18, 17, 15, 14, 12, 10, 9, 7, 5, 3, 6, 1, 3, 5, 7, 9, 6],
        19 => vec![19, 17, 18, 14, 12, 16, 9, 15, 6, 3, 13, 1, 3, 11, 6, 9, 12, 14, 17, 19, 18, 16, 15, 13, 11, 10, 8, 4, 5, 7, 2, 4, 2, 5, 8, 10, 7],
        20 => vec![20, 18, 19, 15, 13, 17, 10, 16, 7, 5, 3, 14, 12, 3, 5, 7, 10, 13, 15, 18, 20, 19, 17, 16, 12, 14, 11, 9, 4, 6, 8, 2, 4, 2, 1, 6, 9, 11, 8],
        _ => vec![],
    }
}
