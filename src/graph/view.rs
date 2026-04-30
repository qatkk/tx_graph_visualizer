use bitcoin::Txid;

/// A structure to enable graph visualization for
///     debugging reasons
pub struct TxGraphView {
    pub nodes: Vec<TxNodeView>,
    pub edges: Vec<TxEdgeView>,
}

/// A structure to enable graph visualization
///     visualizing the graph edges
pub struct TxEdgeView {
    pub from: Txid,
    pub to: Txid,
    pub vout: u32,
    pub amnt: u64,
}

/// A structure to enable graph visualization
///     visualizing the graph nodes
pub struct TxNodeView {
    pub txid: Txid,
    pub is_confirmed: bool,
    //  Transactions that we know of but don't exist in the graph
    pub external: bool,
}
