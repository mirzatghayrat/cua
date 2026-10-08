# Arslan's fork of Cua Driver

This branch (`arslan/cua-driver-rs-v0.34.0`) is upstream tag `cua-driver-rs-v0.34.0`
(`b0968e1b12834e485dda68789541a3cc57664a9f`) plus these commits:

1. **Vendoring** (`52d01740`). Every crate the Cua Driver workspace (`libs/cua-driver/rust`)
   needs on `aarch64-apple-darwin` sits under `libs/cua-driver/rust/vendor/`, and that
   workspace's `.cargo/config.toml` points crates.io at it, so Arslan's release pipeline builds
   the driver with `cargo build --release --locked --offline -p cua-driver` and downloads
   nothing. Made with `cargo vendor-filterer` 0.5.18,
   `--platform=aarch64-apple-darwin --versioned-dirs vendor`, run with upstream's
   `.cargo/config.toml` in place (with ours, cargo would look for the vendor directory being
   made). Crates for other platforms are stubs that keep their `Cargo.toml`, so `cargo metadata`
   and licence checks still see them. Nothing inside a crate is removed: objc2 includes its
   `examples/` in its documentation, and dropping `examples/` broke the build (measured).
2. **No network** (`13c892ba`). `crates/cua-driver/src/arslan.rs` sets `OFFLINE = true`, and the
   four places that would reach the network return early on it: PostHog telemetry
   (`telemetry.rs`: off whatever the environment says, and the sender refuses), the update check
   behind `check_for_update` and the startup banner (`version_check.rs`), skill downloads
   (`skills.rs`) and the self-updater (`updater.rs`). The extension installer only reads local
   archives, so it is left as is: Arslan Hands never offers its tools, nor any tool outside
   Arslan's allow-list.
3. **Two lock bumps for advisories** (`c237a1ca`): `cargo update -p anyhow -p chacha20`, then
   re-vendored the same way. `anyhow` 1.0.102 → 1.0.104 (RUSTSEC-2026-0190, unsound
   `downcast_mut` after `context`) and `chacha20` 0.10.1 → 0.10.2 (0.10.1 is yanked). Nothing
   else in `Cargo.lock` moved, and only those two crates' vendor directories changed. Arslan's
   cargo-deny policy (`packaging/hands/cua-deny.toml`) ignores one advisory, with its reason:
   RUSTSEC-2026-0192, `ttf-parser` is unmaintained and has no release to move to; it parses only
   the font Cua's cursor overlay ships.
4. This file's corrections (the first version described a vendoring command that was not the
   one used, and the third commit's message promised this text before it was written).

Arslan runs the driver only as `cua-driver __private-worker` — a child of its "Arslan Hands"
helper, over pipes, never the socket daemon — with `CUA_DRIVER_EMBEDDED=1`, `DO_NOT_TRACK=1` and
`HOME` pointed at Hands' own folder. Design: `docs/specs/2026-10-08-0157-hands-v2.md` in
[mirzatghayrat/arslan](https://github.com/mirzatghayrat/arslan). Arslan pins this branch's commit
in `packaging/hands/cua-driver.pin`, runs `cargo deny check` on it, and checks every vendored file
against its `.cargo-checksum.json` (`scripts/verify_vendored_checksums.py`).

## Re-vendoring (monthly, or for a security fix)

Read upstream's `cua-driver-rs` releases and the diff of `libs/cua-driver/rust/crates`
(`cua-driver`, `cua-driver-core`, `cua-driver-sdk`, `platform-macos`, `cursor-overlay`). Then: a
new branch from the new tag; re-apply commits 2 and, if still needed, 3; regenerate
`vendor/` with the command above; move the pin; rerun Arslan's contract check and its Hands
harness on a Mac.

Before committing a re-vendor:

- `git status --porcelain --ignored -- libs/cua-driver/rust/vendor` must list nothing ignored
  (`!!`). `libs/cua-driver/rust/.gitignore` re-includes all of `vendor/`.
- No CRLF warnings when staging. The repository's root rule is `* text=auto`, which would rewrite
  CRLF files inside crates on commit; `libs/cua-driver/rust/.gitattributes` sets `vendor/** -text`
  so they are stored byte for byte. Cargo checksums every vendored file, and either slip passes a
  local build (the files on disk are right) and fails only from a clean checkout.
- Build once from a clean clone with an empty `CARGO_HOME`.

## Licence

MIT, unchanged (`LICENSE.md`, `LICENSING.md`). Arslan uses only MIT parts: Cua Driver. Never the
perception extension (its OmniParser model is AGPL-3.0-only) and never Cua Spaces / `cua-spacesd`
(FSL-1.1-MIT).
