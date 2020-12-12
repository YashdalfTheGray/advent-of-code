use std::process;

mod utils;

fn main() {
    let boarding_passes = utils::read_into_collection("input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let mut sanity_checks = vec![];

    for pass in boarding_passes {
        let row_seq = pass.chars().take(7).collect::<String>();
        let col_seq = pass.chars().skip(7).collect::<String>();
        let row = utils::find_position(127, row_seq, 'F', 'B').unwrap();
        let col = utils::find_position(7, col_seq, 'L', 'R').unwrap();
        sanity_checks.push((row * 8) + col);
    }

    let max_sanity_check = sanity_checks.iter().max().unwrap();

    println!(
        "The highest sanity check amongst given boarding passes is {}.",
        max_sanity_check
    );
}
