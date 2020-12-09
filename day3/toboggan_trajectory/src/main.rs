use std::process;

mod path_details;
mod utils;

use path_details::PathDetails;

fn main() {
    let grid = utils::read_into_matrix("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let paths = vec![
        PathDetails::new(1, 1, &grid),
        PathDetails::new(3, 1, &grid),
        PathDetails::new(5, 1, &grid),
        PathDetails::new(7, 1, &grid),
        PathDetails::new(1, 2, &grid),
    ];

    let mut all_the_trees = vec![];

    for path in paths.into_iter() {
        let trees_found = path.number_of_trees();
        all_the_trees.push(trees_found);
        println!(
            "There are {} trees on a {} right {} down path through the given grid.",
            trees_found, path.hslope, path.vslope
        )
    }

    println!(
        "Their total product is {}.",
        all_the_trees.iter().product::<usize>()
    );
}
