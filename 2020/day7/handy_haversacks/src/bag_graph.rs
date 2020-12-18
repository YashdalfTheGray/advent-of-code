use petgraph::{
    dot::{Config, Dot},
    Graph,
};

#[derive(Debug)]
pub struct BagNode {
    pub kind: String,
    pub quantity: u32,
}

impl BagNode {
    pub fn new(kind: &str, quantity: u32) -> BagNode {
        BagNode {
            kind: kind.to_string(),
            quantity,
        }
    }
}

pub fn get_fully_parsed_bag_rules(
    partial_rules: Vec<(String, Vec<String>)>,
) -> Vec<(BagNode, Vec<BagNode>)> {
    partial_rules
        .iter()
        .map(|r| {
            let contained_bags = &r.1;
            let parsed_contained_bags = crate::utils::parse_bag_details(contained_bags.to_vec());

            (&(r.0)[..], parsed_contained_bags)
        })
        .map(|r| {
            let contained_bag_nodes =
                r.1.iter()
                    .map(|r| BagNode::new(&(r.1)[..], r.0))
                    .collect::<Vec<BagNode>>();

            let parent_bag_node = BagNode::new(&r.0.to_string(), 1);

            (parent_bag_node, contained_bag_nodes)
        })
        .collect::<Vec<(BagNode, Vec<BagNode>)>>()
}

pub fn load_into_graph(rules: Vec<(BagNode, Vec<BagNode>)>) -> Graph<String, u32> {
    let mut bags = Graph::<String, u32>::new();

    rules.iter().for_each(|r| {
        let parent = match bags.node_indices().find(|i| bags[*i] == r.0.kind) {
            Some(found_node) => found_node,
            None => bags.add_node(r.0.kind.clone()),
        };

        r.1.iter().for_each(|b| {
            if b.quantity != 0 {
                let bag = bags.add_node(b.kind.clone());
                bags.add_edge(parent, bag, b.quantity);
            }
        })
    });

    bags
}

pub fn dot_format_string(graph: Graph<String, u32>) -> String {
    Dot::with_attr_getters(
        &graph,
        &[Config::EdgeNoLabel],
        &|_, edge| format!("label = \"{}\"", edge.weight()),
        &|_, _| "".to_string(),
    )
    .to_string()
}
