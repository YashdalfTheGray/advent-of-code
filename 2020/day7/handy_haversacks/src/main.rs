#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::process;

mod utils;

fn main() {
    let bag_details = utils::read_into_collection("test-input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let parsed_bag_details = utils::parse_bag_rules(bag_details);
    for d in parsed_bag_details {
        let contained_bags = d.1;
        utils::parse_bag_details(contained_bags)
            .iter()
            .for_each(|b| {
                println!("{:#?}", b);
            })
    }
}
