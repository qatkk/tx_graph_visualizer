use crate::graph::TxGraphView;
use std::string::String;
use std::string::ToString;

impl TxGraphView {
    pub fn to_dot(&self) -> String {
        let mut out = String::new();

        out.push_str("Digraph txgraph {\n");
        out.push_str("graph [bgcolor=white, fontsize=11]; \n");

        for node in &self.nodes {
            if node.external {
                out.push_str(&format!(
                    "\"{}\" [shape=box, color=lightpink, style=filled, label=\"{}...\"];\n",
                    node.txid,
                    &node.txid.to_string()[..8]
                ));
            } else {
                out.push_str(&format!(
                    "\"{}\" [shape=box, color=lightgray, style=filled, label=\"{}...\"];\n",
                    node.txid,
                    &node.txid.to_string()[..8]
                ));
            }
        }

        for edge in &self.edges {
            out.push_str(&format!(
                "\"{}\" -> \"{}\" [fontsize=10, label=\"vout:{}, amnt: {}\"];\n",
                edge.from, edge.to, edge.vout, edge.amnt
            ));
        }

        out.push_str("}\n");

        out
    }
}
