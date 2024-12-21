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
