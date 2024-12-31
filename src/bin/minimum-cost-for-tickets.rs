fn main() {
    println!(
        "{}",
        Solution::mincost_tickets(vec![364], vec![3, 3, 1])
    );
    // println!(
    //     "{}",
    //     Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15])
    // );
}

struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let c1 = costs[0];
        let c7 = costs[1];
        let c30 = costs[2];
        let start_day = days[0] as usize;
        let end_day = days[days.len() - 1] as usize;
        let range = end_day - start_day + 1;
        let mut ans = vec![0; range];
        ans[0] = c1.min(c7).min(c30);
        for i in 1..days.len() {
            let prev_day = days[i - 1] as usize;
            let prev_day_cost = ans[prev_day - start_day];
            let current_day = days[i] as usize;
            let current_day_idx = current_day - start_day;
            for j in (prev_day + 1 - start_day)..(current_day_idx) {
                ans[j] = prev_day_cost;
            }
            let mut current_day_cost = ans[current_day_idx - 1] + c1;
            if current_day < start_day + 7 {
                current_day_cost = current_day_cost.min(c7);
            }
            for t in (if current_day < start_day + 7 { 0 } else { current_day_idx - 7 })..current_day_idx {
                current_day_cost = current_day_cost.min(ans[t] + c7);
            }
            if current_day < start_day + 30 {
                current_day_cost = current_day_cost.min(c30);
            }
            for t in (if current_day < start_day + 30 { 0 } else { current_day_idx - 30 })..current_day_idx {
                current_day_cost = current_day_cost.min(ans[t] + c30);
            }
            ans[current_day_idx] = current_day_cost;
        }
        ans[ans.len() - 1]
    }
}
