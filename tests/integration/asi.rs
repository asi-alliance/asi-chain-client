//! ASI fork acceptance against the selected, immutable Rust-node image.

use libtest_mimic::Trial;

use crate::common::chain;
use crate::common::cli::Cli;
use crate::common::keys;
use crate::common::node::Node;

pub fn trials() -> Vec<Trial> {
    vec![
        Trial::test("asi::native_metadata_is_asi_without_a_signing_key", || {
            for node in Node::all() {
                let status = node.status().expect("a running node's status");
                assert_eq!(status["nativeTokenName"], "ASI");
                assert_eq!(status["nativeTokenSymbol"], "ASI");
                assert_eq!(status["nativeTokenDecimals"], 8);
            }
            Cli::new("status")
                .http_at(Node::ReadOnly)
                .run()
                .expect_success("ASI metadata without a signing key")
                .expect_contains("ASI (ASI, 8 decimals)");
            Cli::new("exploratory-deploy")
                .args([
                    "-f",
                    concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/rho_examples/query_token_metadata.rho"
                    ),
                ])
                .grpc_at(Node::ReadOnly)
                .run()
                .expect_success("on-chain ASI metadata without a signing key")
                .expect_contains("(\"ASI\", \"ASI\", 8)");
            Ok(())
        }),
        Trial::test("asi::whole_tokens_move_exact_asi_dust", || {
            let address = &keys::ASI_TRANSFER_RECIPIENT.address;
            let before = balance(address);
            let sent = Cli::new("transfer")
                .args(["--to-address", address])
                .args(["--amount", "1.00000001", "--whole-tokens"])
                .key(&keys::ASI_TRANSFER_SENDER)
                .at(Node::Validator1)
                .observing(Node::ReadOnly)
                .args(["--max-wait", "120"])
                .run()
                .expect_success("metadata-scaled ASI transfer")
                .expect_contains("100000001 dust")
                .expect_contains("Transfer complete.");
            chain::finalized_on_read_only(&sent);
            assert_eq!(balance(address) - before, 100_000_001);
            Ok(())
        }),
    ]
}

fn balance(address: &str) -> u128 {
    Cli::new("wallet-balance")
        .args(["--address", address])
        .http_at(Node::ReadOnly)
        .run()
        .expect_success("ASI balance without a signing key")
        .value_after(&format!("Balance for {address}:"))
        .parse()
        .expect("an exact base-unit balance")
}
