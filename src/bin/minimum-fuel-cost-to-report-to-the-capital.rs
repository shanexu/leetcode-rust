fn main() {
    println!(
        "{}",
        Solution::minimum_fuel_cost(
            vec![
                vec![3, 1],
                vec![3, 2],
                vec![1, 0],
                vec![0, 4],
                vec![0, 5],
                vec![4, 6],
            ],
            2
        ),
    );
}

struct Solution;
impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        if roads.len() == 0 {
            return 0;
        }
        let n = roads.len() + 1;
        let mut road_map: Vec<Vec<i32>> = vec![vec![];n];
        for road in roads {
            let c1 = road[0];
            let c2 = road[1];
            road_map[c1 as usize].push(c2);
            road_map[c2 as usize].push(c1);
        }
        let mut stack1:Vec<i32> = vec![];
        let mut stack2: Vec<i32> = vec![];

        let mut parents = vec![-1; n];
        stack1.push(0);
        while let Some(c) = stack1.pop() {
            stack2.push(c);
            for &nc in road_map[c as usize].iter() {
                if nc == 0 || parents[nc as usize] != -1 {
                    continue;
                }
                stack1.push(nc);
                parents[nc as usize] = c;
            }
        }
        let mut costs: Vec<i64> = vec![0; n];
        let mut nums = vec![1; n];
        while let Some(c) = stack2.pop() {
            if c == 0 {
                continue;
            }
            let mut cost = costs[c as usize];
            let num = nums[c as usize];
            let mut i = num / seats;
            if i * seats < num {
                i += 1;
            }
            cost += i as i64;
            let parent_city = parents[c as usize];
            let parent_cost  = &mut costs[parent_city as usize];
            let parent_num = &mut nums[parent_city as usize];
            *parent_cost+=cost;
            *parent_num+=num;
        }
        costs[0]
    }
}
