use std::process;

mod answers;
mod utils;

fn main() {
    let customs_answers = utils::read_into_collection("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let unique_answers = answers::find_unique_answers(customs_answers.clone());
    let everyone_answers = answers::find_everyone_answers(customs_answers);

    println!(
        "The sum of all unique answers across all groups in the given input is {}.",
        unique_answers
    );

    println!(
        "The sum of answers that everyone answered in the given input is {}.",
        everyone_answers
    );
}
