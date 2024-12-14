fn main() {
    println!(
        "{:?}",
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![0, 5])
    );
    println!(
        "{:?}",
        Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        )
    );
}

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        let left = new_interval[0];
        let right = new_interval[1];
        let left_index_result = intervals.binary_search_by(|interval| interval[0].cmp(&left));
        let right_index_result = intervals.binary_search_by(|interval| interval[0].cmp(&right));
        let mut left_index = left_index_result.unwrap_or_else(|v| v);
        let mut right_index = right_index_result.unwrap_or_else(|v| v);
        let left_outside_interval = if left_index_result.is_ok() {
            left_index += 1;
            false
        } else if left_index == 0 {
            left < intervals[0][0]
        } else {
            intervals[left_index - 1][1] < left
        };
        let right_outside_interval = if right_index_result.is_ok() {
            right_index += 1;
            false
        } else if right_index == 0 {
            right < intervals[0][0]
        } else {
            intervals[right_index - 1][1] < right
        };
        let mut result: Vec<Vec<i32>> = Vec::new();
        match (left_outside_interval, right_outside_interval) {
            (true, true) => {
                result.extend(intervals[0..left_index].iter().cloned());
                result.push(new_interval);
                result.extend(intervals[right_index..].iter().cloned());
            }
            (true, false) => {
                let new_interval = vec![left, intervals[right_index - 1][1]];
                result.extend(intervals[0..left_index].iter().cloned());
                result.push(new_interval);
                result.extend(intervals[right_index..].iter().cloned());
            }
            (false, true) => {
                let new_interval = vec![intervals[left_index - 1][0], right];
                result.extend(intervals[0..left_index - 1].iter().cloned());
                result.push(new_interval);
                result.extend(intervals[right_index..].iter().cloned());
            }
            (false, false) => {
                let new_interval =
                    vec![intervals[left_index - 1][0], intervals[right_index - 1][1]];
                result.extend(intervals[0..left_index - 1].iter().cloned());
                result.push(new_interval);
                result.extend(intervals[right_index..].iter().cloned());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![0, 1]),
            vec![vec![0, 1], vec![2, 4], vec![6, 8]],
            "case1"
        );
        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![10, 12]),
            vec![vec![2, 4], vec![6, 8], vec![10, 12]],
            "case2"
        );
        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![5, 5]),
            vec![vec![2, 4], vec![5, 5], vec![6, 8]],
            "case3"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![0, 3]),
            vec![vec![0, 4], vec![6, 8]],
            "case4"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![0, 7]),
            vec![vec![0, 8]],
            "case5"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![3, 5]),
            vec![vec![2, 5], vec![6, 8]],
            "case6"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![3, 9]),
            vec![vec![2, 9]],
            "case6"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![3, 3]),
            vec![vec![2, 4], vec![6, 8]],
            "case7"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![3, 6]),
            vec![vec![2, 8]],
            "case8"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![2, 4]),
            vec![vec![2, 4], vec![6, 8]],
            "case9"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![2, 6]),
            vec![vec![2, 8]],
            "case10"
        );

        assert_eq!(
            Solution::insert(vec![vec![2, 4], vec![6, 8]], vec![2, 7]),
            vec![vec![2, 8]],
            "case11"
        );
    }
}
