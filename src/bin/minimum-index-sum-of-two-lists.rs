fn main() {
    assert_eq!(
        Solution::find_restaurant(
            vec![
                "Shogun".to_string(),
                "Tapioca Express".to_string(),
                "Burger King".to_string(),
                "KFC".to_string()
            ],
            vec![
                "Piatti".to_string(),
                "The Grill at Torrey Pines".to_string(),
                "Hungry Hunter Steakhouse".to_string(),
                "Shogun".to_string()
            ]
        ),
        vec!["Shogun".to_string()]
    );
}

struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        for (i, &ref v) in list1.iter().enumerate() {
            map.insert(v, i);
        }
        let mut ans_idx = vec![];
        let mut min_index_sum = list1.len() + list2.len();
        for (j, v) in list2.iter().enumerate() {
            if let Some(&i) = map.get(v) {
                if i + j < min_index_sum {
                    ans_idx.clear();
                    ans_idx.push(j);
                    min_index_sum = i + j;
                } else if i + j == min_index_sum {
                    ans_idx.push(j);
                }
            }
        }
        let mut ans = Vec::with_capacity(ans_idx.len());
        for i in ans_idx {
            ans.push(list2[i].clone());
        }
        ans
    }
}
