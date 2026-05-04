use std::str::FromStr;
use std::sync::Arc;

use bdk_chain::{BlockId, CanonicalizationParams, ConfirmationBlockTime};
use bdk_wallet::coin_selection;
use bdk_wallet::descriptor::{calc_checksum, DescriptorError};
use bdk_wallet::error::CreateTxError;
use bdk_wallet::psbt::PsbtUtils;
use bdk_wallet::signer::{SignOptions, SignerError};
use bdk_wallet::KeychainKind;
use bdk_wallet::{AddressInfo, Balance, PersistedWallet, Update, Wallet, WalletTx};
use bitcoin::constants::COINBASE_MATURITY;
use bitcoin::hashes::Hash;
use bitcoin::script::PushBytesBuf;
use bitcoin::sighash::{EcdsaSighashType, TapSighashType};
use bitcoin::taproot::TapNodeHash;
use bitcoin::{
    absolute, transaction, Address, Amount, BlockHash, FeeRate, Network, OutPoint, ScriptBuf,
    Sequence, SignedAmount, Transaction, TxIn, TxOut, Txid,
};
use rand::rngs::StdRng;
use rand::SeedableRng;

use std::fs;
use tx_graph_visualizer::graph::build_view;
use tx_graph_visualizer::test_utils::*;

#[allow(clippy::print_stdout)]
fn main() {
     let (mut wallet, _) = get_funded_wallet_wpkh();
    let external_tx = Transaction {
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: Txid::all_zeros(), // not owned by wallet
                vout: 0,
            },
            ..Default::default()
        }],
        output: vec![TxOut {
            value: Amount::from_sat(50_000),
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
                vout: 0,
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
            value: Amount::from_sat(45_000),
            script_pubkey: wallet
                .next_unused_address(KeychainKind::Internal)
                .address
                .script_pubkey(),
        }],
        version: transaction::Version::ONE,
        lock_time: absolute::LockTime::ZERO,
    };

    insert_tx(&mut wallet, tx_spend);
    let wallet_balance = wallet.balance();
    println!("the wallet balance is {} ", wallet_balance);

    let tx_graph_vis = build_view(&wallet.tx_graph(), &wallet).to_dot();
    fs::write("graph.dot", tx_graph_vis).unwrap();
}
