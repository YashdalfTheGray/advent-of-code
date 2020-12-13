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

    sanity_checks.sort_unstable();

    let min = sanity_checks[0] as u32;
    let max = sanity_checks[sanity_checks.len() - 1] as u32;
    let mut sum = (((max - min) + 1) / 2) * (max + min);
    sanity_checks.iter().for_each(|&c| {
        sum -= c as u32;
    });

    println!(
        "The largest sanity check in the given boarding passes is {}.",
        max
    );
    println!("The calculated missing sanity check is {}", sum);
}
