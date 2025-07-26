use crate::types::{ArcData, NodeData};
use serde::Serialize;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[tauri::command]
pub fn save_flow_diagram(
    path: String,
    nodes: Vec<NodeData>,
    arcs: Vec<ArcData>,
) -> std::io::Result<()> {
    // `path` passed to this function should include name of flow diagram (e.g. 'my_first_diagram') as suffix
    let mut dir_path = PathBuf::from(path);
    fs::create_dir_all(&dir_path)?; // create directory and parents if they don't exist

    //
    let mut node_file_path: PathBuf = dir_path.clone();
    let mut arc_file_path: PathBuf = dir_path.clone();
    node_file_path.push("nodes.json");
    arc_file_path.push("arc.json");

    // serialise as json
    let node_json = serde_json::to_string_pretty(&nodes).unwrap();
    let arc_json = serde_json::to_string_pretty(&arcs).unwrap();

    // write to files
    let mut node_file = File::create(node_file_path)?;
    let mut arc_file = File::create(arc_file_path)?;
    node_file.write_all(node_json.as_bytes())?;
    arc_file.write_all(arc_json.as_bytes())?;

    return Ok(());
}
