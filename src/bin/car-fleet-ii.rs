fn main() {
    // println!(
    //     "{:?}",
    //     Solution::get_collision_times(vec![vec![1, 2], vec![2, 1], vec![4, 3], vec![7, 2]])
    // );
    // println!(
    //     "{:?}",
    //     Solution::get_collision_times(vec![vec![3, 4], vec![5, 4], vec![6, 3], vec![9, 1]])
    // );
    assert_eq!(
        Solution::get_collision_times(vec![
            vec![7, 5],
            vec![14, 5],
            vec![15, 3],
            vec![16, 4],
            vec![17, 5],
            vec![18, 1]
        ])
        .iter()
        .map(|x| format!("{:.5}", x))
        .collect::<Vec<String>>()
        .join(", "),
        "2.75000, 0.50000, 1.50000, 0.66667, 0.25000, -1.00000"
    );
}

struct Solution;

impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let n = cars.len();
        let mut stack: Vec<usize> = vec![];
        let mut ans = vec![-1.0; n];
        for i in (0..n).rev() {
            while stack.len() > 0 {
                let car0 = &cars[i];
                let i1 = stack[stack.len() - 1];
                let car1 = &cars[i1];
                if car0[1] > car1[1] {
                    let t01 = (car1[0] - car0[0]) as f64 / (car0[1] - car1[1]) as f64;
                    if ans[i1] < 0.0 || t01 <= ans[i1] {
                        ans[i] = t01;
                        break;
                    }
                }
                stack.pop();
            }
            stack.push(i);
        }
        ans
    }
}
