#[derive(Debug)]
pub struct BagNode {
    kind: String,
    quantity: u16,
}

impl BagNode {
    pub fn new(kind: &str, quantity: u16) -> BagNode {
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
