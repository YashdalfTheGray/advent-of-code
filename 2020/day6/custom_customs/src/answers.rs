use std::collections::{HashMap, HashSet};

pub fn find_unique_answers(answers: Vec<String>) -> usize {
    answers
        .iter()
        .map(|s| s.replacen(" ", "", 100))
        .map(|s| {
            let mut result = HashSet::new();
            s.chars().for_each(|c| {
                result.insert(c);
            });
            result
        })
        .map(|hs| hs.len())
        .sum::<usize>()
}

pub fn find_everyone_answers(answers: Vec<String>) -> usize {
    answers
        .iter()
        .map(|s| {
            let parts = s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
            parts
        })
        .map(|parts| {
            let mut answers_map = HashMap::new();
            let mut result = vec![];

            parts.iter().for_each(|s| {
                s.chars().for_each(|c| {
                    *answers_map.entry(c).or_insert(0) += 1;
                })
            });

            answers_map.iter().for_each(|(key, val)| {
                if *val == parts.len() {
                    result.push(*key);
                }
            });

            result
        })
        .map(|r| r.len())
        .sum::<usize>()
}
