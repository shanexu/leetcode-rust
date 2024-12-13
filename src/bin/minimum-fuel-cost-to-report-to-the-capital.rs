fn main() {
    println!(
        "{}",
        Solution2::minimum_fuel_cost(
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

use std::collections::HashMap;
impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        if roads.len() == 0 {
            return 0;
        }
        let mut road_map: HashMap<i32, Vec<i32>> = HashMap::new();
        for road in roads {
            let c1 = road[0];
            let c2 = road[1];
            road_map.entry(c1).or_insert(vec![]).push(c2);
            road_map.entry(c2).or_insert(vec![]).push(c1);
        }
        #[inline]
        fn need_cars(num: i32, seats: i32) -> i32 {
            let i = num / seats;
            if i * seats < num {
                i + 1
            } else {
                i
            }
        }
        fn helper(
            city: i32,
            prev_city: i32,
            road_map: &HashMap<i32, Vec<i32>>,
            seats: i32,
        ) -> (i64, i32) {
            let mut cost = 0;
            let mut num = 1;
            for &next_city in &road_map[&city] {
                if prev_city != next_city {
                    let (c, n) = helper(next_city, city, road_map, seats);
                    cost += c;
                    cost += need_cars(n, seats) as i64;
                    num += n;
                }
            }
            (cost, num)
        }
        let mut cost = 0;
        for &c in road_map[&0].iter() {
            let (c, n) = helper(c, 0, &road_map, seats);
            cost += c;
            cost += need_cars(n, seats) as i64;
        }
        cost
    }
}

struct Solution2;
impl Solution2 {
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
        let mut values: Vec<(i64, i32)> = vec![(0, 1); n];
        while let Some(c) = stack2.pop() {
            if c == 0 {
                continue;
            }
            let (mut cost, mut num) =  values[c as usize];
            let mut i = num / seats;
            if i * seats < num {
                i += 1;
            }
            cost += i as i64;
            let parent_city = parents[c as usize];
            let pair: &mut (i64, i32) = &mut values[parent_city as usize];
            *pair = (pair.0 + cost, pair.1 + num);
        }
        values[0].0
    }
}
