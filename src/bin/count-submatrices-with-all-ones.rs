fn main() {
    println!(
        "{}",
        Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]])
    );
}

struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat[0].len();
        let mut arr = vec![0; n];
        let mut count = 0;
        for row in mat {
            for i in 0..n {
                if row[i] == 0 {
                    arr[i] = 0;
                } else {
                    arr[i] += 1;
                }
            }
            count += sum_subarray_mins(&arr);
        }
        count
    }
}

fn sum_subarray_mins(arr: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut s = 0;
    let mut stack: Vec<usize> = vec![];
    for (i, &v) in arr.iter().enumerate() {
        while !stack.is_empty() && arr[stack[stack.len() - 1]] > v {
            let j = stack.pop().unwrap();
            let width = if stack.is_empty() {
                j + 1
            } else {
                j - stack[stack.len() - 1]
            };
            s -= (width as i32) * arr[j];
        }
        let width = if stack.is_empty() {
            i + 1
        } else {
            i - stack[stack.len() - 1]
        };
        s += (width as i32) * v;
        sum += s;
        stack.push(i);
    }
    sum
}
