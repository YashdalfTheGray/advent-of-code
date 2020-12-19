#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::process;

mod bag_graph;
mod utils;

fn main() {
    let filename = "test-input";
    let bag_to_start_with = "shiny gold";

    let bag_details = utils::read_into_collection(filename).unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });
    let partial_bag_rules = utils::parse_bag_rules(bag_details);
    let parsed_bag_rules = bag_graph::get_fully_parsed_bag_rules(partial_bag_rules);

    let bags = bag_graph::load_into_graph(parsed_bag_rules);
    let bags_string = bag_graph::dot_format_string(bags.clone());

    utils::write_string_to_file(bags_string, "graph.dot".to_string());

    let start = bags
        .node_indices()
        .find(|i| bags[*i] == bag_to_start_with)
        .unwrap();
    let containing = bag_graph::count_containing_bags(bags.clone(), start);
    let contained = bag_graph::count_bags_contained(bags, start);

    println!(
        "The number of bags that can contain at least one {} bag is {}.",
        bag_to_start_with, containing
    );
    println!(
        "The total number of bags contained within a {} bag is {}",
        bag_to_start_with, contained
    )
}
