use std::process;

mod utils;

fn main() {
    let contents = utils::read_and_process("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    println!(
        "There were {} passwords that match the range scheme.",
        contents.0
    );
    println!(
        "There were {} passwords that match the location scheme.",
        contents.1
    );
}
