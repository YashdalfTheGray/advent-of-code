use std::process;

mod searchers;
mod utils;

fn main() {
    let contents = utils::read_contents("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let pair = searchers::find_pair_with_sum(&contents, 2020);

    println!(
        "Found {} and {} in the given list that add up to 2020.\nTheir product is {}.",
        pair.0,
        pair.1,
        pair.0 * pair.1
    );
}
