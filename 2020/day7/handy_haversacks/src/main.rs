#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::process;

mod bag_graph;
mod utils;

fn main() {
    let bag_details = utils::read_into_collection("test-input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });
    let partial_bag_rules = utils::parse_bag_rules(bag_details);
    let parsed_bag_rules = bag_graph::get_fully_parsed_bag_rules(partial_bag_rules);

    let mut bags = bag_graph::load_into_graph(parsed_bag_rules);
    bags.reverse();

    let bags_string = bag_graph::dot_format_string(bags.clone());
    println!("Is the graph directed? {}", bags.is_directed());

    utils::write_string_to_file(bags_string, "graph.dot".to_string());
}
