use crate::graph::TxGraphView;
use std::string::String;
use std::string::ToString;

impl TxGraphView {
    pub fn to_dot(&self) -> String {
        let mut out = String::new();

        out.push_str("Digraph txgraph {\n");
        out.push_str("graph [bgcolor=white, fontsize=11]; \n");
        out.push_str(
            "label=\"Transaction graph built from BDK wallet data
            Pink Node: external transaction not included in the graph
            Gray Node: non-canonical transactions
            Green Node: confirmed canonical transactions 
            Yellow Node: unconfirmed canonical transactions
            Self loops (Black): UTXOs in the canonical chain
            Self loops (Pink): External or internal non-canonical outputs \n \"; 
            labelloc=\"t\"; \n",
        );
        for node in &self.nodes {
            if node.external {
                out.push_str(&format!(
                    "\"{}\" [shape=box, color=lightpink, style=filled, label=\"{}...\"];\n",
                    node.txid,
                    &node.txid.to_string()[..8]
                ));
            } else {
                if node.canonical && node.is_confirmed {
                    out.push_str(&format!(
                        "\"{}\" [shape=box, color=lightgreen, style=filled, label=\"{}...\"];\n",
                        node.txid,
                        &node.txid.to_string()[..8]
                    ));
                } else if node.canonical && !node.is_confirmed {
                    out.push_str(&format!(
                        "\"{}\" [shape=box, color=yellow, style=filled, label=\"{}...\"];\n",
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
        }

        for edge in &self.edges {
            if edge.external {
                out.push_str(&format!(
                    "\"{}\" -> \"{}\" [fontsize=10, color=lightpink, label=\"vout:{}, amnt: {}\"];\n",
                    edge.from, edge.to, edge.vout, edge.amnt
                ));
            } else {
                out.push_str(&format!(
                    "\"{}\" -> \"{}\" [fontsize=10, label=\"vout:{}, amnt: {}\"];\n",
                    edge.from, edge.to, edge.vout, edge.amnt
                ));
            }
        }

        out.push_str("}\n");

        out
    }
}
