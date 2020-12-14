use std::{
    collections::{HashMap, HashSet},
    process,
};

mod utils;

fn main() {
    let customs_answers = utils::read_into_collection("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let unique_answers = customs_answers
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
        .collect::<Vec<usize>>();

    let everyone_answers = customs_answers
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
        .sum::<usize>();

    println!(
        "The sum of all unique answers across all groups in the given input is {}.",
        unique_answers.iter().sum::<usize>()
    );

    println!(
        "The sum of answers that everyone answered in the given input is {}.",
        everyone_answers
    );
}
