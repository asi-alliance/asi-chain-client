//! ASI behavior that must survive upstream synchronization.

use node_cli::vault::{dust_to_tokens, parse_amount, tokens_to_dust, validate_address};
use node_cli::{Endpoint, F1r3flyApi};

#[test]
fn asi_base_units_are_exact_and_never_rounded() {
    let amount = parse_amount("100000001").unwrap();
    assert_eq!(dust_to_tokens(&amount, 8).unwrap(), "1.00000001");
    assert_eq!(tokens_to_dust("1.00000001", 8).unwrap(), amount);
    assert!(tokens_to_dust("1.000000001", 8).is_err());
}

#[test]
fn metadata_precision_is_not_hardcoded_to_eight() {
    let amount = tokens_to_dust("1.000000000000000001", 18).unwrap();
    assert_eq!(amount.to_string(), "1000000000000000001");
    assert_eq!(dust_to_tokens(&amount, 18).unwrap(), "1.000000000000000001");
}

#[test]
fn readonly_api_requires_no_private_key_or_connection() {
    let endpoint = Endpoint::parse("http://127.0.0.1:40401").unwrap();
    assert!(F1r3flyApi::new(endpoint).is_ok());
}

#[test]
fn public_address_validation_rejects_non_base58_characters() {
    // A valid prefix and length must not admit an invalid base58 payload.
    let invalid = format!("1111{}", "0".repeat(48));
    assert!(validate_address(&invalid).is_err());
    #[cfg(feature = "cli")]
    assert!(node_cli::commands::validate_vault_address(&invalid).is_err());
}
