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

    let mut deps = Graph::<&str, &str>::new();
    let pg = deps.add_node("petgraph");
    let fb = deps.add_node("fixedbitset");
    let qc = deps.add_node("quickcheck");
    let rand = deps.add_node("rand");
    let libc = deps.add_node("libc");
    deps.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);

    println!("{:#?}", parsed_bag_rules);

    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));
}
