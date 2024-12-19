fn main() {}

/// O(n)
struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut stack = vec![];
        for x in arr {
            if stack.is_empty() || stack[stack.len() - 1] <= x {
                stack.push(x);
            } else {
                let cur_max = stack.pop().unwrap();
                while !stack.is_empty() && stack[stack.len() - 1] > x {
                    stack.pop();
                }
                stack.push(cur_max);
            }
        }
        stack.len() as i32
    }
}

/// O(nlog(n))
struct Solution2;

impl Solution2 {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        let mut count = 0;
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            let pv = map.entry(v).or_insert(0);
            *pv += 1;
            if *pv == 0 {
                map.remove(&v);
            }
            let u = sorted_arr[i];

            let pu = map.entry(u).or_insert(0);
            *pu -= 1;
            if *pu == 0 {
                map.remove(&u);
            }

            if map.is_empty() {
                count += 1;
            }
        }
        count
    }
}
