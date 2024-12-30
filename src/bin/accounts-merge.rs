fn main() {}

struct Solution;

use std::collections::{BTreeSet, HashMap};
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
