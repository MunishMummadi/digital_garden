use petgraph::graph::Graph;
use petgraph::dot::{Dot, Config};
use std::collections::HashMap;
use crate::storage::{garden_path, load_note};
use std::fs;

pub fn visualize_notes() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = Graph::<String, ()>::new();
    let mut node_map = HashMap::new();

    for entry in fs::read_dir(garden_path())? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let note = load_note(path.file_stem().unwrap().to_str().unwrap())?;
            let node_index = graph.add_node(note.title.clone());
            node_map.insert(note.title.clone(), node_index);
        }
    }

    for entry in fs::read_dir(garden_path())? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let note = load_note(path.file_stem().unwrap().to_str().unwrap())?;
            let from_index = node_map[&note.title];
            for link in note.links {
                if let Some(&to_index) = node_map.get(&link) {
                    graph.add_edge(from_index, to_index, ());
                }
            }
        }
    }

    let dot = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    
    // Save the DOT representation to a file
    let dot_path = garden_path().join("note_graph.dot");
    fs::write(&dot_path, format!("{:?}", dot))?;
    println!("Graph visualization saved to {:?}", dot_path);

    // If you have Graphviz installed, you can use this command to convert DOT to PNG:
    // dot -Tpng note_graph.dot -o note_graph.png
    println!("To create a PNG image, run: dot -Tpng {:?} -o note_graph.png", dot_path);

    Ok(())
}