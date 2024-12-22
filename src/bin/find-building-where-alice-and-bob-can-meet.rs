fn main() {
    println!(
        "{:?}",
        Solution::leftmost_building_queries(
            vec![2, 4, 1, 3],
            vec![
                // vec![0, 0],
                // vec![0, 1],
                vec![0, 2],
                // vec![0, 3],
                // vec![1, 0],
                // vec![1, 1],
                // vec![1, 2],
                vec![1, 3],
                // vec![2, 0],
                // vec![2, 1],
                // vec![2, 2],
                // vec![2, 3],
                // vec![3, 0],
                // vec![3, 1],
                // vec![3, 2],
                // vec![3, 3]
            ]
        )
    );
}

struct Solution;

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn search(height: i32, stack: &Vec<(i32, usize)>) -> Option<usize> {
            if stack.is_empty() {
                return None;
            }
            let mut size = stack.len();
            let mut left = 0;
            let mut right = size;
            let mut result = None;
            while left < right {
                let mid = left + size / 2;
                if stack[mid].0 > height {
                    result = result.map(|a| std::cmp::max(a, mid)).or(Some(mid));
                    left = mid + 1;
                } else {
                    right = mid;
                }

                size = right - left;
            }
            result.filter(|i| *i < stack.len())
        }

        let mut ans = vec![-1; queries.len()];
        let mut new_queries = vec![];
        for (i, q) in queries.iter().enumerate() {
            let mut x = q[0];
            let mut y = q[1];
            if x == y {
                ans[i] = y;
                continue;
            }
            if x > y {
                std::mem::swap(&mut x, &mut y);
            }
            if heights[x as usize] < heights[y as usize] {
                ans[i] = y;
                continue;
            }
            new_queries.push((x, y, i));
        }
        new_queries.sort_by(|a, b| a.1.cmp(&b.1));
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut prev_y = (heights.len() - 1) as i32;
        for &(x, y, i) in new_queries.iter().rev() {
            if prev_y != y {
                for j in (y + 1..=prev_y).rev() {
                    while !stack.is_empty() && heights[j as usize] >= stack[stack.len() - 1].0 {
                        stack.pop();
                    }
                    stack.push((heights[j as usize], j as usize));
                }
            }
            if let Some(k) = search(heights[x as usize], &stack) {
                ans[i] = stack[k].1 as i32
            }
            prev_y = y;
        }
        ans
    }
}
