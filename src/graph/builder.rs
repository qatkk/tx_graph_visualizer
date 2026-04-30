use crate::graph::view::{TxEdgeView, TxGraphView, TxNodeView};
use bdk_chain::TxGraph;
use bdk_wallet::Wallet;
use bitcoin::{OutPoint, Txid, hashes::Hash};
use std::collections::HashSet;

pub fn build_view(tx_graph: &TxGraph, wallet: &Wallet) -> TxGraphView {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let full_txs_vec = tx_graph.full_txs().collect::<Vec<_>>();
    let all_tx_outptuts: HashSet<OutPoint> = tx_graph
        .all_txouts()
        .enumerate()
        .map(|(_, op)| op.0)
        .collect();
    let mut visited_tx_outputs: HashSet<OutPoint> = HashSet::new();

    for tx_node in full_txs_vec.iter() {
        nodes.push(TxNodeView {
            txid: tx_node.txid,
            is_confirmed: !tx_node.anchors.is_empty(),
            external: false,
        });
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
                    nodes.push(TxNodeView {
                        txid: Txid::all_zeros(),
                        is_confirmed: false,
                        external: true,
                    });
                    0
                },
            });
            visited_tx_outputs.insert(input.previous_output);
        }
    }
    // Adding the unspent outputs to the graph
    for (_, unspent_output) in wallet.list_unspent().enumerate() {
        edges.push(TxEdgeView {
            from: unspent_output.outpoint.txid,
            to: unspent_output.outpoint.txid,
            vout: unspent_output.outpoint.vout,
            amnt: unspent_output.txout.value.to_sat(),
        });
        visited_tx_outputs.insert(unspent_output.outpoint);
    }
    for (_, external_paid_outputs) in all_tx_outptuts.difference(&visited_tx_outputs).enumerate() {
        nodes.push(TxNodeView {
            txid: Txid::all_zeros(),
            is_confirmed: false,
            external: true,
        });
        edges.push(TxEdgeView {
            from: external_paid_outputs.txid,
            to: Txid::all_zeros(),
            vout: external_paid_outputs.vout,
            amnt: tx_graph
                .get_txout(*external_paid_outputs)
                .unwrap()
                .value
                .to_sat(),
        });
    }

    TxGraphView { nodes, edges }
}
