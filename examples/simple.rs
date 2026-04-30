use bdk_wallet::KeychainKind;
use tx_graph_visualizer::graph::build_view;
use tx_graph_visualizer::test_utils::*;
use bitcoin::{
    Amount, OutPoint, Transaction, TxIn, TxOut, absolute, transaction
};
use std::fs; 


#[allow(clippy::print_stdout)]
fn main() {
    let (mut wallet, _) = get_funded_wallet_wpkh();
    let outpoint = wallet.list_unspent().next().unwrap().outpoint;
    let external_tx = Transaction {
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: outpoint.txid, // not owned by wallet
                vout: outpoint.vout,
            },
            ..Default::default()
        }],
        output: vec![TxOut {
            value: Amount::from_sat(40_000),
            script_pubkey: wallet
                .next_unused_address(KeychainKind::External)
                .address
                .script_pubkey()
        }, TxOut {
            value: Amount::from_sat(5_000),
            script_pubkey: wallet
                .next_unused_address(KeychainKind::External)
                .address
                .script_pubkey()
        }],
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
    };
    let external_txid = external_tx.compute_txid();
    insert_tx(&mut wallet, external_tx);

    let tx_spend_1 = Transaction {
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: external_txid , // not owned by wallet
                vout: 1,
            },
            ..Default::default()
        }],
        output: vec![TxOut {
            value: Amount::from_sat(5_000),
            script_pubkey: wallet
                .next_unused_address(KeychainKind::Internal)
                .address
                .script_pubkey()
        }],
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
    };

    insert_tx(&mut wallet, tx_spend_1);

    let tx_spend = Transaction {
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: external_txid,
                vout: 0,
            },
            ..Default::default()
        }],
        output: vec![TxOut {
            value: Amount::from_sat(35_000),
            script_pubkey: wallet
                .next_unused_address(KeychainKind::Internal)
                .address
                .script_pubkey(),
        }],
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
    };

    insert_tx(&mut wallet, tx_spend);

    let tx_graph_vis = build_view(&wallet.tx_graph()).to_dot();
    fs::write("graph.dot", tx_graph_vis).unwrap();
}
