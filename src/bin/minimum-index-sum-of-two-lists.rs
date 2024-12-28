fn main() {

}

struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        for (i, &ref v) in list1.iter().enumerate() {
            map.insert(v, i);
        }
        let mut ans = vec![];
        let mut min_index_sum = list1.len() + list2.len();
        for (j, v) in list2.iter().enumerate() {
            if let Some(&i) = map.get(v) {
                if i + j < min_index_sum {
                    ans.clear();
                    ans.push(j);
                    min_index_sum = i + j;
                } else if i + j == min_index_sum {
                    ans.push(j);
                }
            }
        }
        ans.iter().map(|&i| list2[i].clone()).collect()
    }
}