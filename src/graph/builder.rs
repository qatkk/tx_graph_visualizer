use crate::graph::view::{TxEdgeView, TxGraphView, TxNodeView};
use bdk_chain::{CanonicalizationParams, ChainOracle};
use bdk_wallet::Wallet;
use bitcoin::{OutPoint, Txid};
use std::collections::HashSet;

pub fn build_view(wallet: &Wallet) -> TxGraphView {
    let tx_graph = wallet.tx_graph();
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let full_txs_vec = tx_graph.full_txs().collect::<Vec<_>>();
    let all_tx_outputs: HashSet<OutPoint> = tx_graph
        .all_txouts()
        .enumerate()
        .map(|(_, op)| op.0)
        .collect();
    let canonical_tx: HashSet<Txid> = tx_graph
        .list_canonical_txs(
            wallet.local_chain(),
            wallet.local_chain().get_chain_tip().unwrap(),
            CanonicalizationParams::default(),
        )
        .enumerate()
        .map(|(_index, canon_tx)| canon_tx.tx_node.txid)
        .collect();
    let mut visited_tx_outputs: HashSet<OutPoint> = HashSet::new();

    for tx_node in full_txs_vec.iter() {
        nodes.push(TxNodeView {
            txid: tx_node.txid,
            is_confirmed: !tx_node.anchors.is_empty(),
            external: false,
            canonical: canonical_tx.contains(&tx_node.txid),
        });
        // Adding transactions to the graph as nodes
        // Adding spent outputs to the graph
        for input in tx_node.tx.input.iter() {
            edges.push(TxEdgeView {
                from: input.previous_output.txid,
                to: tx_node.txid,
                vout: input.previous_output.vout,
                amnt: if let Some(prev_tx_node) = tx_graph.get_tx_node(input.previous_output.txid) {
                    prev_tx_node
                        .tx_out(input.previous_output.vout as usize)
                        .unwrap()
                        .value
                        .to_sat()
                } else {
                    // Transactions we know of but that do not exist in our graph
                    nodes.push(TxNodeView {
                        txid: input.previous_output.txid,
                        is_confirmed: false,
                        external: true,
                        canonical: false,
                    });
                    0
                },
                external: false,
            });
            visited_tx_outputs.insert(input.previous_output);
        }
    }
    // Adding the unspent outputs in the canonical chain
    for (_, unspent_output) in wallet.list_unspent().enumerate() {
        edges.push(TxEdgeView {
            from: unspent_output.outpoint.txid,
            to: unspent_output.outpoint.txid,
            vout: unspent_output.outpoint.vout,
            amnt: unspent_output.txout.value.to_sat(),
            external: false,
        });
        visited_tx_outputs.insert(unspent_output.outpoint);
    }
    // Adding unspent outputs outside of the canonical chain
    //  These includes two different scenarios
    //      1: We're paying to external addresses that we don't own, and they're unspent
    //      2: Unspent outputs belonging to the wallet but not in the canonical chain
    for (_, unvisited_output) in all_tx_outputs.difference(&visited_tx_outputs).enumerate() {
        edges.push(TxEdgeView {
            from: unvisited_output.txid,
            to: unvisited_output.txid,
            vout: unvisited_output.vout,
            amnt: tx_graph
                .get_txout(*unvisited_output)
                .unwrap()
                .value
                .to_sat(),
            external: true,
        });
    }

    TxGraphView { nodes, edges }
}
