use std::process;

mod passport_details;
mod utils;

fn main() {
    let passport_collection = utils::read_into_collection("test-input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    for passport in passport_collection.iter() {
        println!("{:?}", passport);
    }
}
