fn main() {
    println!(
        "{:?}",
        Solution::accounts_merge(vec_vec_string![
            ["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            ["John", "johnsmith@mail.com", "john00@mail.com"],
            ["Mary", "mary@mail.com"],
            ["John", "johnnybravo@mail.com"]
        ])
    );
    println!(
        "{:?}",
        Solution::accounts_merge(vec_vec_string![
            ["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
            ["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
            ["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
            ["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
            ["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"]
        ])
    );
}

struct Solution;

use std::collections::{BTreeSet, HashMap};

use leetcode_rust::vec_vec_string;
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        fn find(id: usize, parent: &mut Vec<usize>) -> usize {
            if parent[id] != id {
                parent[id] = find(parent[id], parent);
            }
            parent[id]
        }

        let mut parent: Vec<usize> = (0..accounts.len()).collect();
        let mut email_to_idx: HashMap<&String, usize> = HashMap::new();
        for i in 0..accounts.len() {
            let account = &accounts[i];
            for j in 1..account.len() {
                let email = &account[j];
                if let Some(&t) = email_to_idx.get(email) {
                    let ft = find(t, &mut parent);
                    let fi = find(i, &mut parent);
                    parent[fi] = ft;
                } else {
                    email_to_idx.insert(email, i);
                }
            }
        }

        let mut merged_accounts: HashMap<usize, BTreeSet<&String>> = HashMap::new();
        for i in 0..accounts.len() {
            let account = &accounts[i];
            for j in 1..account.len() {
                let email = &account[j];
                let root = find(i, &mut parent);
                merged_accounts
                    .entry(root)
                    .or_insert_with(BTreeSet::new)
                    .insert(email);
            }
        }

        let mut result = vec![];
        for (id, emails) in merged_accounts {
            let mut account: Vec<String> = vec![];
            account.push(accounts[id][0].clone());
            for email in emails {
                account.push(email.to_string())
            }
            result.push(account);
        }
        result
    }
}
