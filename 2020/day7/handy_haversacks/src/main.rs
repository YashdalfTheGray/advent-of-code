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

    let partial_bag_rules = utils::parse_bag_rules(bag_details);

    let parsed_bag_rules = partial_bag_rules
        .iter()
        .map(|r| {
            let contained_bags = &r.1;
            let parsed_contained_bags = utils::parse_bag_details(contained_bags.to_vec());
            (&(r.0)[..], parsed_contained_bags)
        })
        .map(|r| (r.0.to_string(), r.1))
        .collect::<Vec<(String, Vec<(u16, String)>)>>();

    println!("{:#?}", parsed_bag_rules);
}
