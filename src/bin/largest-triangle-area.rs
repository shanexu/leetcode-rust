fn main() {
    println!(
        "{}",
        Solution::largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0]
        ])
    );
}

struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let mut ans = 0.0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    let a = triangle_area(&points[i], &points[j], &points[k]);
                    if a > ans {
                        ans = a;
                    }
                }
            }
        }
        ans
    }
}

#[inline]
fn triangle_area(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> f64 {
    0.5 * f64::abs(
        a[0] as f64 * (b[1] - c[1]) as f64
            + b[0] as f64 * (c[1] - a[1]) as f64
            + c[0] as f64 * (a[1] - b[1]) as f64,
    )
}
