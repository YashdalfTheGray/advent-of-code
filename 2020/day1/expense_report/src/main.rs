use std::process;

mod utils;

fn main() {
    let contents = utils::read_contents("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let set_of_numbers = utils::load_into_set(&contents);

    let mut found_number = 0;
    for n in contents {
        if set_of_numbers.contains(&(2020 - n)) {
            found_number = n;
            break;
        }
    }

    println!(
        "Found {} and {} in the given list that add up to 2020.\nTheir product is {}",
        found_number,
        2020 - found_number,
        found_number * (2020 - found_number)
    );
}
