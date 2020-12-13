use std::process;

mod utils;

fn main() {
    let customs_answers = utils::read_into_collection("test-input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    customs_answers.iter().for_each(|a| {
        println!("{}", a);
    })
}
