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
