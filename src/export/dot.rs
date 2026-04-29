use crate::graph::TxGraphView;
use std::string::String;
use std::string::ToString;
use std::fs;

impl TxGraphView {
    pub fn to_dot(&self) -> String {
        let mut out = String::new();

        out.push_str("Digraph txgraph {\n");

        for node in &self.nodes {
            out.push_str(&format!("\"{}\" [label=\"{}\"];\n", node.txid, &node.txid.to_string()[..8]));
        }

        for edge in &self.edges {
            out.push_str(&format!("\"{}\" -> \"{}\" [label=\"{}\"];\n", edge.from, edge.to, edge.vout));
        }

        out.push_str("}\n");

        out
    }
}
pub fn write_dot_file(path: &str, graph: &TxGraphView){
    let _ = fs::write(path, graph.to_dot());
}