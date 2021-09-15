fn main() {
    println!("{:?}", Solution::permute_unique(vec![1, 1, 2, 2]))
}

struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut results = Vec::with_capacity(size(&nums));
        let mut result = vec![i32::MIN; nums.len()];
        solve(&mut results, &mut result, &nums, 0);
        results
    }
}

fn factorial(n: usize) -> usize {
    let mut p = 1;
    for i in 1..n+1 {
        p = p*i;
    }
    p
}

fn size(nums: &Vec<i32>) -> usize {
    let mut n = factorial(nums.len());
    let mut p = i32::MIN;
    let mut c = 0;
    for i in 0..nums.len() {
        let x = nums[i];
        if x == p {
            c+=1;
        } else {
            n = n / factorial(c);
            c = 1;
            p = x;
        }
    }
    n / factorial(c)
}

fn solve(results: &mut Vec<Vec<i32>>, result: &mut Vec<i32>, nums: &Vec<i32>, step: usize) {
    if step == nums.len() - 1 {
        for i in 0..result.len() {
            if result[i] == i32::MIN {
                result[i] = nums[step];
                results.push(result.clone());
                result[i] = i32::MIN;
                break;
            } else if nums[step] == result[i] {
                break;
            }
        }
        return;
    }

    for i in 0..result.len() {
        if result[i] == i32::MIN {
            result[i] = nums[step];
            solve(results, result, nums, step + 1);
            result[i] = i32::MIN;
        } else if nums[step] == result[i] {
            break;
        }
    }
}
