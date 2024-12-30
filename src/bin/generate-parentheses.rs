fn main() {
    println!("{:?}", Solution::generate_parenthesis(1));
    println!("{:?}", Solution::generate_parenthesis(2));
    println!("{:?}", Solution::generate_parenthesis(3));
    println!("{:?}", Solution::generate_parenthesis(4));
    println!("{:?}", Solution::generate_parenthesis(5));
    println!("{:?}", Solution::generate_parenthesis(6));
    println!("{:?}", Solution::generate_parenthesis(7));
    println!("{:?}", Solution::generate_parenthesis(8));
}

struct Solution {}

/*
f(1) = ["()"]
f(2) = ["()()", "(())"]
f(3) = ["()()()", "(())()", "()(())", "(()())", "((()))"]
*/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        gen(0, n, n, String::from(""), &mut results);
        results
    }
}

fn gen(
    sum: i32,
    left_count: i32,
    right_count: i32,
    current_str: String,
    results: &mut Vec<String>,
) {
    if right_count > 0 && sum + 1 <= 0 {
        let mut new_current_str = String::with_capacity(current_str.len() + 1);
        new_current_str.clone_from(&current_str);
        new_current_str.insert(current_str.len(), ')');
        gen(
            sum + 1,
            left_count,
            right_count - 1,
            new_current_str,
            results,
        );
    }
    if left_count > 0 {
        let mut new_current_str = String::with_capacity(current_str.len() + 1);
        new_current_str.clone_from(&current_str);
        new_current_str.insert(current_str.len(), '(');
        gen(
            sum - 1,
            left_count - 1,
            right_count,
            new_current_str,
            results,
        );
    }
    if left_count == 0 && right_count == 0 {
        results.push(current_str);
    }
}
