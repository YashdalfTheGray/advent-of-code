#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::process;

use petgraph::{
    dot::{Config, Dot},
    Graph,
};

mod bag_graph;
mod utils;

fn main() {
    let bag_details = utils::read_into_collection("test-input").unwrap_or_else(|err| {
        println!("{:?}", err);
        process::exit(1)
    });

    let partial_bag_rules = utils::parse_bag_rules(bag_details);

    let parsed_bag_rules = bag_graph::get_fully_parsed_bag_rules(partial_bag_rules);

    let mut bags = Graph::<String, u32>::new();
    let root = bags.add_node("Start".to_string());

    parsed_bag_rules.iter().for_each(|r| {
        let parent = bags.add_node(r.0.kind.clone());
        bags.add_edge(root, parent, r.0.quantity);

        r.1.iter().for_each(|b| {
            if b.quantity != 0 {
                let bag = bags.add_node(b.kind.clone());
                bags.add_edge(parent, bag, b.quantity);
            }
        })
    });

    println!(
        "{:?}",
        Dot::with_attr_getters(
            &bags,
            &[Config::EdgeNoLabel],
            &|_, edge| format!("label = \"{}\"", edge.weight()),
            &|_, node| format!("label = \"{}\"", node.1.to_lowercase())
        )
    );
}
