# Contributing to rust-client

Thanks for contributing! This repo follows a `dev → main` flow.

## ASI fork

This is `asi-alliance/asi-chain-client`. The owner selected `dev` as the base
for synchronization PR #3. The integration preserves the historical ASI
`asi-main` changes and the maintained upstream history.

During this transition the GitHub default branch remains `asi-main`; select
`dev` explicitly when opening development PRs. The intended release promotion
is `dev` to `main`, where the Release workflow runs. Default-branch, protection
and legacy-branch migration are separate maintainer decisions; this PR does
not change repository settings or authorize a release. Keep `asi-main` and
`asi-dev` available while their remaining unique behavior is reviewed.

Send generic upstream fixes to `F1R3FLY-io/rust-client`'s `dev`. See
[ASI migration](docs/asi-migration.md) for required ASI behavior and the selected
node image. Consumer gitlinks and published images require separate changes.

## Branching policy

```
feature/your-change ──► dev ──► main
```

- **All feature/fix PRs target `dev`.** Select it explicitly while the fork default is still `asi-main`.
- **Maintainers** cut releases by merging `dev` into `main`.

Use `dev` to `main` promotion PRs for reviewed releases; feature/fix PRs target `dev`. Confirm the actual fork protection and release settings before promotion.

## Opening a PR

1. Fork the repo (or create a feature branch if you have write access).
2. Create your branch from the latest `dev`:
   ```bash
   git checkout dev
   git pull origin dev
   git checkout -b feature/short-description
   ```
3. Make your changes. Keep commits focused; write clear messages.
4. Update documentation as needed — `README.md`, files under `docs/`, command-specific docs in `docs/commands/`, and any inline doc comments. Docs and code should ship in the same PR.
5. Push and open a PR. Explicitly select `dev` as the base branch during the fork transition.

## Quality checks

Before opening a PR:

- `cargo build --release` passes
- `cargo fmt --check` passes (run `cargo fmt` if not)
- `cargo clippy --all-targets -- -D warnings` passes
- `cargo test` passes (unit tests)
- For changes that touch node interaction, also run the integration suite,
  which brings up its own shard (requires Docker):
  ```bash
  cargo test --release --features integration --test integration
  ```

CI runs these on every PR. Failing CI blocks merge.

## Versioning & releases

A merge to `main` releases through the Release workflow — `dev` does not release. The workflow releases the next patch version unless `Cargo.toml` on the branch declares a higher version, which is how a minor or major release is chosen: a reviewed commit on `dev` setting `version`. Do not edit `CHANGELOG.md`; the release pipeline writes it.

Release tags must remain immutable. Confirm the ASI fork's tag ruleset and workflow permissions before release; upstream ruleset descriptions do not establish the fork's configuration. A version that was ever released must not be released again, even when its tag build failed: that version stays without a release, and the fix ships as the next one.

## Code style

- Follow rustfmt defaults
- Prefer `Option<T>` over sentinel values
- Avoid silent error swallowing — propagate or log
- Match the existing module layout (`src/commands/`, `src/grpc/`, etc.)

## Questions

Open a discussion or draft PR if you're unsure about scope or approach.
