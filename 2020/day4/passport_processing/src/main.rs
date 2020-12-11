use std::process;

mod passport_details;
mod utils;

fn main() {
    let passport_collection = utils::read_into_collection("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let valid_passports = passport_collection
        .iter()
        .map(|p| p.is_valid())
        .filter(|v| *v)
        .count();

    println!(
        "There were {} valid passports in the given batch file.",
        valid_passports
    );
}
