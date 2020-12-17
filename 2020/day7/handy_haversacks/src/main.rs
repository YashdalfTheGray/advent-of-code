#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::process;

use petgraph::dot::{Config, Dot};

mod bag_graph;
mod utils;

fn main() {
    let bag_details = utils::read_into_collection("test-input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });
    let partial_bag_rules = utils::parse_bag_rules(bag_details);
    let parsed_bag_rules = bag_graph::get_fully_parsed_bag_rules(partial_bag_rules);
    let bags = bag_graph::load_into_graph(parsed_bag_rules);

    println!(
        "{:?}",
        Dot::with_attr_getters(
            &bags,
            &[Config::EdgeNoLabel],
            &|_, edge| format!("label = \"{}\"", edge.weight()),
            &|_, _| "".to_string()
        )
    );
}
