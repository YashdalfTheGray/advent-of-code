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
        "Found {} and {} to add up to 2020 in the list.\nTheir product is {}.",
        pair.0,
        pair.1,
        pair.0 * pair.1
    );

    let triplet = searchers::find_triplets_with_sum(&contents, 2020);

    println!(
        "Found {}, {}, and {} to add up to 2020 in the list.\nTheir product is {}.",
        triplet.0,
        triplet.1,
        triplet.2,
        triplet.0 * triplet.1 * triplet.2
    )
}
