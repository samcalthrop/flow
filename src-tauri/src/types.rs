use serde::{Deserialize, Serialize};

// diagram data
#[derive(Serialize, Deserialize)]
pub struct NodeData {
    pub id: String,
    pub position: Position,
    pub data: NodeLabel,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize)]
pub struct NodeLabel {
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArcData {
    pub id: String,
    pub source: String,
    pub target: String,
}
