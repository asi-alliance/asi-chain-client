# ASI client synchronization

## Sources

The synchronization combines ASI `asi-main` at
`4166a32f562c4e2a85ba10ec2d7cf16451834596` and upstream v0.2.9 at
`4bb001690b2893e0cd465f71feac3ed9e3a1b81d`.

All four node crates (`models`, `shared`, `crypto`, `rholang`) use maintained
`F1R3FLY-io/f1r3node-rust` source `5dafbc37dad3ebf3da810641fc0d8cecd5a1127b`.
This is also the source of the existing linux/amd64 acceptance image:

```text
public.ecr.aws/l2u2v4a8/asi-chain/node-rust@sha256:e6e82cdc5c9bc794367e15314d9c22256a8ebc0b2457d9492d633b2453c4c1df
```

The older `asi-chain-rust-node` fork follows the deprecated combined parent;
it is not this image's source. Tests use the existing image and do not build
a node. Client compilation builds shared dependency crates only.

## Conflict decisions

| ASI behavior / old conflict | Resolution in the new architecture |
|---|---|
| `args`, dispatcher, endpoint flags | Keep v0.2.9 endpoint URLs, transport settings and explicit signing keys. Retire the old argument structs instead of maintaining a second CLI parser. |
| Read-only API, query/events commands | `F1r3flyApi::new(Endpoint)` starts without a signing key; HTTP queries use `NodeHttp`. No dummy private key. |
| REV-to-ASI and token names | Keep the actual current `rho:vault:system` / `rho:system:deployerId` ABI. Read native name, symbol and decimals from the selected node; do not restore old `revVault` / `asiVault` URIs or hardcode F1R3. |
| Embedded contracts | Vault and PoS builders embed their Rholang in the library. They do not read `rho_examples/cli` at runtime. Old templates are superseded by these builders. |
| Transfer amounts | Raw dust by default, exact integers through uint256. `--whole-tokens` uses node precision without floating-point rounding. Eight-decimal ASI remains supported. |
| Transfer/status/output | Keep typed upstream lifecycle errors and positive contract-result checks. Inclusion, finalization and execution are distinct; a timeout is unknown outcome, not successful execution or permission to send a new transfer. |
| Public address validator / crypto conflict | Preserve `commands::validate_vault_address` as a public wrapper around the library validator, including base58 checks. Keep the maintained crypto implementation. |
| README and network guide | Use current upstream documentation with ASI authority, migration and exact-image notes. |

`ticker_rename` is historical evidence. Do not merge it wholesale or replay all
old ASI commits: much of their behavior already exists in newer modules.

## Consumer migration

This is an API upgrade from ASI 0.1.1 to the v0.2.9 architecture. Existing
published wallet CLI/faucet artifacts keep their existing client pins.
Adapt each consumer and review its new exact ASI client pin separately.

| Old caller | Current API |
|---|---|
| `F1r3flyApi::new(key, host, port)` / `new_readonly` | `F1r3flyApi::new(Endpoint::parse(url)?)?`, then optional `with_signing_key` |
| `TransferArgs` with host and port fields | `ConnectionConfig` and `F1r3flyConnectionManager` for library calls; `--grpc`, `--http`, `--observer` URLs for CLI calls |
| `transfer_command` returning a compressed status tuple | `manager.transfer(address, amount_dust)` for a verified transfer; typed errors retain deploy IDs for uncertain outcomes |
| `Deploying` after wait timeout | Handle `Error::FinalizationTimeout` as an unknown result and poll the existing deploy ID; never generate a replacement payment automatically |
| Fixed eight-decimal display | `dust_to_tokens` / `tokens_to_dust` using the node's decimals; `wallet-balance` prints exact raw base units |
| CLI key argument / implicit development key | `SigningKey` or `--private-key-file` / `FIREFLY_PRIVATE_KEY`; no default key |

See [library usage](library/getting-started.md), [transfer](commands/transfer.md)
and [deploy status](commands/deploy-status.md). Consumer adaptation is a required
step before selecting new faucet/wallet images; this PR does not update them.

## Acceptance

The integration fixture uses ASI / ASI / 8 metadata, newly generated per-run
keys and isolated chain state. It checks native metadata without a signing key
and an exact `1.00000001 ASI` transfer of `100000001` dust. The rest of the
upstream suite checks raw-unit transfers, rejected transfers, deploy lifecycle,
TLS/routing and validator lifecycle against the same image.

```bash
cargo fmt --check
cargo clippy --all-targets --features cli,integration --locked -- -D warnings
cargo build --release --locked
cargo test --release --locked
cargo test --release --no-default-features --locked
cargo test --release --features integration --test integration --locked
```

The immutable node image is amd64. CI builds and unit-tests both client
architectures, but runs image-backed acceptance on amd64. No existing network,
genesis or validator keys are modified by these tests.
