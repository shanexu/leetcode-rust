fn main() {}

struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();
        let mut cars = Vec::with_capacity(n);
        for i in 0..n {
            cars.push((position[i], speed[i]));
        }
        cars.sort();
        let mut stack = vec![];
        for (pos, v) in cars {
            let t = (target - pos) as f64 / v as f64;
            while !stack.is_empty() && t >= stack[stack.len() - 1] {
                stack.pop();
            }
            stack.push(t);
        }
        stack.len() as i32
    }
}

/// 用了桶排序
struct Solution2;

impl Solution2 {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut ts = vec![0.0; target as usize];
        let mut stack = vec![];
        let n = position.len();
        for i in 0..n {
            let p = position[i];
            let d = target - p;
            let v = speed[i];
            let t = d as f64 / v as f64;
            ts[p as usize] = t;
        }
        for t in ts {
            if t == 0.0 {
                continue;
            }
            while !stack.is_empty() && t >= stack[stack.len() - 1] {
                stack.pop();
            }
            stack.push(t);
        }
        stack.len() as i32
    }
}
