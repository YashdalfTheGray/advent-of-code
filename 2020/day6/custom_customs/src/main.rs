use std::{collections::HashSet, process};

mod utils;

fn main() {
    let customs_answers = utils::read_into_collection("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let unique_answers = customs_answers
        .iter()
        .map(|s| {
            let mut result = HashSet::new();
            s.chars().for_each(|c| {
                result.insert(c);
            });
            result
        })
        .map(|hs| hs.len())
        .collect::<Vec<usize>>();

    println!(
        "The sum of all unique answers across all groups in the given input is {}",
        unique_answers.iter().sum::<usize>()
    )
}
