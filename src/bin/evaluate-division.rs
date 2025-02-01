fn main() {
    println!(
        "{:?}",
        Solution::calc_equation(
            vec![
                vec!["x1".to_string(), "x2".to_string()],
                vec!["x2".to_string(), "x3".to_string()],
                vec!["x1".to_string(), "x4".to_string()],
                vec!["x2".to_string(), "x5".to_string()]
            ],
            vec![3.0, 0.5, 3.4, 5.6],
            vec![
                vec!["x2".to_string(), "x4".to_string()],
                vec!["x1".to_string(), "x5".to_string()],
                vec!["x1".to_string(), "x3".to_string()],
                vec!["x5".to_string(), "x5".to_string()],
                vec!["x5".to_string(), "x1".to_string()],
                vec!["x3".to_string(), "x4".to_string()],
                vec!["x4".to_string(), "x3".to_string()],
                vec!["x6".to_string(), "x6".to_string()],
                vec!["x0".to_string(), "x0".to_string()]
            ]
        )
    );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut parent: HashMap<&str, (f64, &str)> = HashMap::new();
        let n = equations.len();
        for e in equations.iter() {
            parent.insert(&e[0], (1.0, &e[0]));
            parent.insert(&e[1], (1.0, &e[1]));
        }
        for i in 0..n {
            let e = &equations[i];
            let v = values[i];
            let a = find(&e[0], &mut parent).unwrap();
            let b = find(&e[1], &mut parent).unwrap();
            let c = (b.0 / a.0 * v, b.1);
            parent.insert(a.1, c);
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries.iter() {
            let v = match (find(&q[0], &mut parent), find(&q[1], &mut parent)) {
                (Some((x, a)), Some((y, b))) => {
                    if a == b {
                        x / y
                    } else {
                        -1.0
                    }
                }
                _ => -1.0,
            };
            ans.push(v);
        }
        ans
    }
}

fn find<'a>(x: &'a str, parent: &mut HashMap<&'a str, (f64, &'a str)>) -> Option<(f64, &'a str)> {
    if let Some(p) = parent.get(x).cloned() {
        if p.1 != x {
            if let Some(pp) = find(p.1, parent) {
                parent.insert(x, (pp.0 * p.0, pp.1));
            } else {
                return None;
            }
        }
        return parent.get(x).cloned();
    }
    None
}
