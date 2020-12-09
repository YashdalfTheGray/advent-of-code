use std::process;

mod utils;

fn main() {
    let grid = utils::read_into_matrix("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let path = utils::traverse(grid, 3, 1);

    let num_of_trees = path.into_iter().filter(|e| *e == 'X').count();

    println!(
        "There are {} trees on a 3 right, 1 down path through the given grid.",
        num_of_trees
    );
}
