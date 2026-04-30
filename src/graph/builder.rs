use bdk_chain::TxGraph;
use bdk_wallet::Wallet;
use bitcoin::{Txid, hashes::Hash};
use crate::graph::view::{TxGraphView, TxNodeView, TxEdgeView};

pub fn build_view(tx_graph: &TxGraph, wallet: &Wallet) -> TxGraphView {
    let mut nodes = Vec::new(); 
    let mut edges = Vec::new();
    let full_txs_vec= tx_graph.full_txs().collect::<Vec<_>>();

    for tx_node in full_txs_vec.iter() {
        nodes.push(TxNodeView{
            txid: tx_node.txid,
            is_confirmed: !tx_node.anchors.is_empty(), 
            external: false,
        });
        for input in tx_node.tx.input.iter() {
            edges.push(TxEdgeView{
                from: input.previous_output.txid,
                to: tx_node.txid,
                vout: input.previous_output.vout,
                amnt: if let Some(prev_tx_node) = tx_graph
                    .get_tx_node(input.previous_output.txid){
                        prev_tx_node.tx_out(input.previous_output.vout as usize)
                        .unwrap()
                        .value.to_sat()
                    } else {
                        nodes.push(TxNodeView { 
                            txid: Txid::all_zeros(), 
                            is_confirmed: false,
                            external: true,
                        });
                        0
                    }
            });
        }
    }
    // Adding the unspent outputs to the graph
    for (_, unspent_output) in wallet.list_unspent().enumerate(){
        edges.push(TxEdgeView{
            from: unspent_output.outpoint.txid,
            to: unspent_output.outpoint.txid,
            vout: unspent_output.outpoint.vout,
            amnt: unspent_output.txout.value.to_sat()
        });
    }

    TxGraphView { 
        nodes, 
        edges 
    }
}
