# ASI Chain Node CLI

Rust library and CLI for ASI Chain's Rust nodes, maintained as a fork of
[F1R3FLY-io/rust-client](https://github.com/F1R3FLY-io/rust-client).

This synchronization starts from upstream v0.2.9 and preserves ASI native-token
behavior using the current system-vault ABI. Read the [ASI migration guide](docs/asi-migration.md)
before upgrading from 0.1.1: the library API and endpoint flags have changed.
Consumer gitlinks must be updated separately after adapting their callers.

## Branches and testnet builds

Branch roles verified on 2026-11-03:

| Branch | Role |
|---|---|
| `dev` | Active ASI integration and development base. [PR #3](https://github.com/asi-alliance/asi-chain-client/pull/3) merged here as `40f2a9408040252dd23761cd06adb9f1407ef228`, preserving the reviewed ASI runtime integration `77cbdc8`. |
| `main` | Release line, currently upstream v0.2.9. Promotion of the ASI integration from dev is a separate maintainer decision; the Release workflow runs on main. |
| `asi-main` | Historical ASI 0.1.1 line; still the GitHub default during the transition. Select dev explicitly for current development. |
| `asi-dev` | Older separate ASI line with unique OpenSSL/target configuration; preserve it for review, but do not use it as the current integration baseline. |

Clone the active integration explicitly:

```bash
git clone --branch dev https://github.com/asi-alliance/asi-chain-client.git
cd asi-chain-client
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for development and release promotion.

## Quick Start

Node endpoints are URLs: `http://` is plaintext, `https://` is TLS (add `--ca-cert <PEM>` to trust a private CA; it is refused if any endpoint of the command is `http://`). An HTTP endpoint may carry a base path, such as `https://proxy.example/node1`; a gRPC endpoint may not. `--connect-timeout` (default 10) and `--timeout` (default 30) bound, in seconds, connecting and each request. Commands default to a node's own ports, gRPC `http://localhost:40401` and HTTP `http://localhost:40403`, except `propose` (internal gRPC, `http://localhost:40402`) and commands that need a read-only node, which require the endpoint flag. Every setting can also come from an environment variable or a named profile in a config file; see [Configuration](docs/configuration.md). The examples below target the system-integration single-host Docker shard.

```bash
# Build
cargo build --release --locked

# Check node status and native token info
cargo run --release -- status --http http://localhost:40413

# Deploy to validator1 and wait for the result, also polling the read-only observer
cargo run --release -- deploy-and-wait -f contract.rho \
  --grpc http://localhost:40412 --http http://localhost:40413 \
  --observer http://localhost:40453

# Read-only query
cargo run --release -- exploratory-deploy -f query.rho --grpc http://localhost:40452

# Query native token metadata from on-chain contract
cargo run --release -- exploratory-deploy -f rho_examples/query_token_metadata.rho --grpc http://localhost:40452
```

## Local development

Commands that sign need a key. It is never a flag value: pass `--private-key-file <PATH>` (env `FIREFLY_PRIVATE_KEY_FILE`), a file holding the key's hex that group and others cannot read, or set `FIREFLY_PRIVATE_KEY` to the hex. With neither, the command exits with an error.

```bash
node_cli generate-key-pair --save --output-dir ~/.keys    # private_key.hex, mode 0600
export FIREFLY_PRIVATE_KEY_FILE=~/.keys/private_key.hex
```

Local dev/test Docker setups fund a well-known development key, which ships in
those Docker configs — take it from the node setup you are running against
(`.env.node` in system-integration, `docker/.env` in f1r3node-rust) rather than
from here. Never reuse a key from a README against anything real.

## Documentation

### Commands
- [deploy](docs/commands/deploy.md) -- submit Rholang code to the blockchain
- [deploy-and-wait](docs/commands/deploy-and-wait.md) -- deploy, wait for finalization, read result
- [exploratory-deploy](docs/commands/exploratory-deploy.md) -- read-only Rholang execution
- [estimate-cost](docs/commands/estimate-cost.md) -- estimate phlogiston cost before deploying
- [get-deploy](docs/commands/get-deploy.md) -- get deploy execution details
- [deploy-status](docs/commands/deploy-status.md) -- canonical finalization status of a deploy signature
- [get-data](docs/commands/get-data.md) -- read deploy result data
- [propose](docs/commands/propose.md) -- manually propose a block
- [is-finalized](docs/commands/is-finalized.md) -- check block finalization
- [transfer](docs/commands/transfer.md) -- transfer native tokens
- [Node inspection](docs/commands/inspection.md) -- status, blocks, bonds, balance, etc.
- [Key management](docs/commands/keys.md) -- generate keys, addresses
- [Configuration](docs/configuration.md) -- config file profiles, environment variables, signing keys, `config show`
- [Advanced](docs/commands/advanced.md) -- load-test, watch-events, dag, bond-validator, unbond-validator, PoS queries

### Guides
- [Bonding a validator](docs/guides/bonding-a-validator.md) -- fund, bond, activate, unbond, payout

### Library
- [Getting started](docs/library/getting-started.md) -- ConnectionManager API, config, examples
- [Events](docs/library/events.md) -- WebSocket deploy finalization
- [Architecture](docs/architecture.md) -- module structure, deploy flow, node endpoints

### Testing
- [Testing guide](docs/testing.md) -- unit tests, and the integration suite that runs against a shard it creates

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `FIREFLY_PRIVATE_KEY_FILE` | One of these two | -- | File holding the signing key's hex; refused if group or others can read it |
| `FIREFLY_PRIVATE_KEY` | One of these two | -- | Signing key (64 hex chars) |
| `FIREFLY_GRPC_URL` | No | `http://localhost:40401` | Deploy node's gRPC endpoint |
| `FIREFLY_HTTP_URL` | No | `http://localhost:40403` | Deploy node's HTTP endpoint |
| `FIREFLY_OBSERVER_URL` | No | none | HTTP endpoint of a second node polled for the deploy's verdict; without it, only the deploy node is polled |
| `FIREFLY_CA_CERT` | No | none | PEM file of the certificate authorities trusted for every `https://` endpoint, in place of the bundled public roots |
| `FIREFLY_CONNECT_TIMEOUT` | No | `10` | Seconds a connection may take to establish (at least 1) |
| `FIREFLY_REQUEST_TIMEOUT` | No | `30` | Seconds a single request may take (at least 1) |
| `FIREFLY_FINALIZATION_TIMEOUT` | No | `90` | Max seconds to wait for a deploy to finalize |
| `FIREFLY_SHARD_ID` | No | `root` | Shard deploys are signed for: the node's `casper.shard-name` |

The library's `ConnectionConfig::from_env` reads these, and fails if both key variables are set. The CLI reads the same variables as fallbacks for its flags (except `FIREFLY_FINALIZATION_TIMEOUT`), also refuses a key file together with `FIREFLY_PRIVATE_KEY`, and adds `FIREFLY_INTERNAL_GRPC_URL`, `FIREFLY_READONLY_GRPC_URL`, `FIREFLY_READONLY_HTTP_URL`, `FIREFLY_CONFIG` and `FIREFLY_PROFILE`; see [Configuration](docs/configuration.md).

An unset variable takes its default; a set but malformed one, such as a URL without an `http` or `https` scheme, is an error. So is `FIREFLY_CA_CERT` with any `http://` endpoint, and any removed variable (`FIREFLY_HOST`, `FIREFLY_GRPC_PORT`, `FIREFLY_HTTP_PORT`, `FIREFLY_OBSERVER_HOST`, `FIREFLY_OBSERVER_GRPC_PORT`, `FIREFLY_OBSERVER_HTTP_PORT`); the error names its replacement.

## Deploy admission refusals

For nodes with [bounded admission (#789)](https://github.com/F1R3FLY-io/f1r3node-rust/pull/789), `submit` returns `Error::DeployAdmissionRefused` with the original signed deploy ID and `DeployAdmissionReason::PoolBusy` or `AdmissionBusy` when the existing gRPC service-error envelope explicitly reports that code. These requests were not accepted. Callers may wait and resubmit the same `SignedDeploy`; `deploy` creates a newly signed deploy. No automatic submission retry is added.

Transport timeouts and unrecognised service errors retain their existing types and do not prove non-acceptance. `submit_and_wait` returns a typed admission refusal without starting finalization polling. Reconcile an unknown outcome by the original deploy ID before deciding to submit again.
