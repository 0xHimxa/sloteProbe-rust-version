# 🛠 SlotProbe — 2-Week Build Plan (Rust)

> **Your situation:** Chapter 12 of The Rust Book, 2.5 hours/day, building v1 in Rust.
> **Start date:** July 10, 2026 — **End date:** July 23, 2026
> **Total time budget:** ~35 hours (14 days × 2.5 hrs)

---

## 📚 What You Already Know (Chapters 1–12)

Based on being at Chapter 12, you're solid on:
- Variables, types, functions, control flow
- Ownership, borrowing, references, slices
- Structs, enums, `match`, `Option`
- Packages, crates, modules, `use`
- Common collections (Vec, String, HashMap)
- Error handling (`Result`, `?`, `unwrap`)
- Generics, traits, lifetimes (basics)
- Writing tests (`#[test]`, `assert!`, `assert_eq!`)
- The I/O project (file reading, CLI args, `std::env`)

## 🚨 What You Still Need (Chapters 13–20 + Ecosystem)

| Concept | Where You'll Hit It | What to Study |
|---|---|---|
| **Closures & Iterators** (Ch 13) | Day 2 — filtering variables, `.map()`, `.filter()`, `.collect()` chains | [Rust Book Ch 13](https://doc.rust-lang.org/book/ch13-00-functional-features.html) |
| **Cargo & crates.io** (Ch 14) | Day 1 — adding dependencies, `Cargo.toml` features | [Rust Book Ch 14](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) |
| **Smart Pointers** (Ch 15) | Day 7 — `Box<dyn Error>`, trait objects in formatters | [Rust Book Ch 15](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) |
| **Concurrency** (Ch 16) | Day 10 — not directly used (we use async instead), but concepts help | [Rust Book Ch 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) |
| **Async/Await** (not in book) | Day 3 — every RPC call is async via `tokio` | [Tokio Tutorial](https://tokio.rs/tokio/tutorial) (first 3 sections) |
| **Trait Objects** (Ch 17) | Day 7 — `Box<dyn Formatter>` for output layer | [Rust Book Ch 17](https://doc.rust-lang.org/book/ch17-00-oop.html) |
| **Derive Macros** (Ch 19) | Day 1 — `#[derive(Serialize, Deserialize, Parser)]` | [Serde guide](https://serde.rs/derive.html), [Clap derive](https://docs.rs/clap/latest/clap/_derive/index.html) |
| **Serde** (ecosystem) | Day 2 — JSON parsing for artifacts | [Serde.rs](https://serde.rs/) |
| **Clap** (ecosystem) | Day 1 — CLI argument parsing | [Clap docs](https://docs.rs/clap/latest/clap/) |
| **Alloy** (ecosystem) | Day 3 — RPC client, EVM primitives | [Alloy Book](https://alloy.rs/) |
| **Handlebars** (ecosystem) | Day 11 — migration script generation | [handlebars-rust](https://docs.rs/handlebars/latest/handlebars/) |

---

## 🗓 The Plan — Day by Day

---

### 📅 Day 1 (July 10) — Project Setup + Clap CLI Skeleton

**⏱ Time: 2.5 hours**

**🧠 LEARN FIRST (30 min):** Before coding, read these:
- [Rust Book Ch 14 — Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html) — you need to understand `Cargo.toml` features, adding deps
- [Clap Derive Tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html) — just the derive API tutorial (15 min read)
- [Serde Derive Overview](https://serde.rs/derive.html) — just the derive page (5 min read)

**What to build:**
- [ ] `cargo new slotprobe --bin`
- [ ] Add all dependencies to `Cargo.toml`:
  ```toml
  [dependencies]
  tokio = { version = "1", features = ["full"] }
  alloy = { version = "0.9", features = ["full"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  clap = { version = "4", features = ["derive"] }
  indicatif = "0.17"
  owo-colors = "4"
  comfy-table = "7"
  anyhow = "1"
  thiserror = "2"
  ```
- [ ] Create the module structure (empty `mod.rs` files):
  ```
  src/
  ├── main.rs
  ├── cli/
  │   ├── mod.rs
  │   └── commands/
  │       ├── mod.rs
  │       ├── snapshot.rs
  │       ├── diff.rs
  │       ├── check_collision.rs
  │       └── generate_migration.rs
  ├── core/
  │   ├── mod.rs
  │   ├── storage_engine/
  │   │   ├── mod.rs
  │   │   ├── reader.rs
  │   │   ├── slot_calculator.rs
  │   │   ├── decoder.rs
  │   │   └── packed.rs
  │   ├── artifact_parser/
  │   │   ├── mod.rs
  │   │   ├── foundry.rs
  │   │   ├── hardhat.rs
  │   │   ├── normalizer.rs
  │   │   └── types.rs
  │   ├── snapshot/
  │   │   ├── mod.rs
  │   │   ├── capture.rs
  │   │   ├── store.rs
  │   │   └── types.rs
  │   ├── diff/
  │   │   ├── mod.rs
  │   │   ├── engine.rs
  │   │   └── types.rs
  │   ├── collision/
  │   │   ├── mod.rs
  │   │   ├── detector.rs
  │   │   └── report.rs
  │   └── migration/
  │       ├── mod.rs
  │       └── generator.rs
  ├── rpc/
  │   ├── mod.rs
  │   ├── client.rs
  │   └── chains.rs
  └── config/
      ├── mod.rs
      └── schema.rs
  ```
- [ ] Implement `main.rs` with Clap — just the subcommand skeleton (no logic, just args parsing):
  ```
  slotprobe snapshot <address> --chain <chain> --block <num> --artifact <path> --only <vars> --out <path> --dry-run
  slotprobe diff <before> <after> --output <format>
  slotprobe check-collision <old> <new>
  slotprobe generate-migration <before> <after> --format <fmt> --out <path> --verify --dry-run
  ```
- [ ] `cargo build` — make sure it compiles with all deps

**✅ Milestone:** `cargo run -- --help` prints all subcommands. `cargo run -- snapshot --help` prints snapshot flags.

---

### 📅 Day 2 (July 11) — Artifact Parser Types + Foundry Parser

**⏱ Time: 2.5 hours**

**🧠 LEARN FIRST (40 min):** This day is heavy on new Rust concepts:
- [Rust Book Ch 13 — Closures and Iterators](https://doc.rust-lang.org/book/ch13-00-functional-features.html) — you'll use `.iter().map().filter().collect()` constantly from here on. **Read the full chapter.**
- [Serde.rs — Using Derive](https://serde.rs/derive.html) + [Attributes](https://serde.rs/attributes.html) — how `#[derive(Serialize, Deserialize)]` works, `#[serde(rename)]`, `#[serde(default)]`

**EVM Knowledge (20 min):**
- Read [Solidity Docs — Layout of State Variables in Storage](https://docs.soliditylang.org/en/latest/internals/layout_in_storage.html) — **first pass**, focus on: what a slot is, how `uint256` takes one full slot, how smaller types pack.
- Skim a Foundry artifact JSON (`out/Contract.sol/Contract.json`) — look at the `storageLayout` key structure.

**What to build:**
- [ ] `src/core/artifact_parser/types.rs` — define `StorageVariable`, `StorageLayout`, `TypeInfo` structs with serde derives
- [ ] `src/core/artifact_parser/foundry.rs` — parse a Foundry JSON artifact file, extract `storageLayout.storage` into your `StorageLayout` struct
- [ ] Write unit tests using a fixture JSON file (copy a real Foundry artifact into `tests/fixtures/`)

**✅ Milestone:** `cargo test` passes. You can parse a real Foundry artifact and print every variable name + slot number.

---

### 📅 Day 3 (July 12) — RPC Client + Slot Reader (First Async Code)

**⏱ Time: 2.5 hours**

**🧠 LEARN FIRST (45 min) — This is your biggest learning day:**
- [Tokio Tutorial — Hello Tokio + Spawning](https://tokio.rs/tokio/tutorial/hello-tokio) — understand `#[tokio::main]`, `async fn`, `.await`
- [Alloy Getting Started](https://alloy.rs/) — how to create a provider, make a call
- [Alloy `get_storage_at` example](https://github.com/alloy-rs/examples) — find the provider examples

**What to build:**
- [ ] `src/rpc/client.rs` — `get_client(rpc_url) -> Result<RootProvider>` using alloy's `ProviderBuilder`
- [ ] `src/rpc/chains.rs` — a simple mapping of chain name → default RPC URL
- [ ] `src/core/storage_engine/reader.rs` — `async fn read_slot(address, slot, rpc_url, block_number) -> Result<B256>`
- [ ] Update `main.rs` to use `#[tokio::main]`
- [ ] Write a small test/example that reads slot 0 of a known contract (you can use a public RPC for now)

**💡 Tip:** If you don't have an Alchemy/Infura key yet, get a free one from [Alchemy](https://www.alchemy.com/) — you'll need it for all RPC calls.

**✅ Milestone:** You can run a small binary that reads a single storage slot from mainnet and prints the raw hex value.

---

### 📅 Day 4 (July 13) — Slot Calculator + Type Decoder

**⏱ Time: 2.5 hours**

No new Rust chapters today! You already know everything needed. This is pure EVM knowledge day.

**EVM Knowledge (30 min):**
- Re-read [Solidity Storage Layout](https://docs.soliditylang.org/en/latest/internals/layout_in_storage.html) — this time focus on:
  - **Mappings:** `keccak256(abi.encode(key, slot))` — how keys get hashed
  - **Dynamic arrays:** length at slot N, elements at `keccak256(N) + index`
  - **Packed storage:** how `uint128 + uint128` share one slot
- [RareSkills — EVM Storage](https://www.rareskills.io/post/evm-solidity-storage-layout) — excellent diagrams

**What to build:**
- [ ] `src/core/storage_engine/slot_calculator.rs`:
  - `fn mapping_slot(key: Address, base_slot: U256) -> U256` — keccak256 hashing
  - `fn array_element_slot(base_slot: U256, index: U256) -> U256`
  - `fn nested_mapping_slot(keys: &[Address], base_slot: U256) -> U256`
- [ ] `src/core/storage_engine/decoder.rs`:
  - `fn decode_value(raw: B256, solidity_type: &str) -> serde_json::Value`
  - Handle: `bool`, `uint*`, `int*`, `address`, `bytes*`
- [ ] `src/core/storage_engine/packed.rs`:
  - `fn extract_packed_value(raw_slot: B256, byte_offset: u32, num_bytes: u32) -> B256`
- [ ] **Unit tests** for all of the above — use known values you can verify with `cast storage`

**✅ Milestone:** `cargo test` — all decoder tests pass. You can compute the correct mapping slot for a known address and verify it manually.

---

### 📅 Day 5 (July 14) — Snapshot Capture + Serialization

**⏱ Time: 2.5 hours**

No new Rust concepts. You're combining what you built on Days 2–4.

**What to build:**
- [ ] `src/core/snapshot/types.rs` — `Snapshot`, `SnapshotEntry` structs with serde
- [ ] `src/core/snapshot/capture.rs`:
  - `async fn capture_snapshot(address, layout, chain, rpc_url, block_number) -> Result<Snapshot>`
  - Iterate over `StorageLayout.variables`, call `read_slot` + `decode_value` for each
  - Handle packed slots (multiple variables sharing one slot — read once, extract each)
- [ ] `src/core/snapshot/store.rs`:
  - `fn save_snapshot(snapshot: &Snapshot, path: &Path) -> Result<()>` — `serde_json::to_string_pretty`
  - `fn load_snapshot(path: &Path) -> Result<Snapshot>`
- [ ] Wire the `snapshot` CLI command: parse args → load artifact → capture snapshot → save to file

**💡 Tip:** Install Foundry (`curl -L https://foundry.paradigm.xyz | bash && foundryup`) and use `anvil --fork-url <your_rpc>` for local testing. This gives you a free local fork of mainnet.

**✅ Milestone:** `cargo run -- snapshot 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 --chain mainnet --artifact ./tests/fixtures/usdc.json --out snap.json` produces a JSON file with named variables and decoded values.

---

### 📅 Day 6 (July 15) — Snapshot Filters + Dry-Run + Hardhat Parser

**⏱ Time: 2.5 hours**

No new Rust concepts.

**What to build:**
- [ ] `--only` flag filter logic:
  - Filter `StorageLayout.variables` to only include named variables from the comma-separated list
  - Print helpful error if a variable name doesn't exist in the layout
- [ ] `--dry-run` mode for snapshot:
  - Print "Would read N variables across M RPC calls" without actually calling the RPC
  - Print the list of variables that would be read
- [ ] `--mapping-keys` support:
  - Load a JSON file of `{ "variableName": ["0xkey1", "0xkey2"] }`
  - For each key, compute the mapping slot and add it to the read list
- [ ] `src/core/artifact_parser/hardhat.rs` — parse Hardhat artifact format
- [ ] `src/core/artifact_parser/normalizer.rs` — auto-detect Foundry vs Hardhat and delegate

**✅ Milestone:** `cargo run -- snapshot 0x... --only fee,tickSpacing --dry-run` prints what it would read. Hardhat artifacts parse correctly.

---

### 📅 Day 7 (July 16) — Diff Engine + Terminal Output

**⏱ Time: 2.5 hours**

**🧠 LEARN FIRST (30 min):**
- [Rust Book Ch 15 — Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) — focus on `Box<T>`. You'll use `Box<dyn Formatter>` for the output layer.
- [Rust Book Ch 17.2 — Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) — `dyn Trait` syntax, when to use it
- [owo-colors README](https://docs.rs/owo-colors/latest/owo_colors/) — terminal color API (2 min)
- [comfy-table README](https://docs.rs/comfy-table/latest/comfy_table/) — table output (5 min)

**What to build:**
- [ ] `src/core/diff/types.rs` — `DiffStatus` enum (`Added`, `Removed`, `Changed`, `Unchanged`), `DiffEntry`, `DiffResult`
- [ ] `src/core/diff/engine.rs` — `fn diff_snapshots(before: &Snapshot, after: &Snapshot) -> DiffResult`
  - Build a HashMap from `after`, iterate `before`, detect added/removed/changed/unchanged
  - Compute summary counts
- [ ] `src/cli/formatters/terminal.rs`:
  - Colored output: red for removed, green for added, dim for unchanged
  - Format like `git diff` style
- [ ] Wire the `diff` CLI command: load two snapshot files → diff → print

**✅ Milestone:** `cargo run -- diff before.json after.json` prints a beautiful colored diff in the terminal.

---

### 📅 Day 8 (July 17) — JSON + Markdown Output Formatters

**⏱ Time: 2.5 hours**

No new Rust concepts. This is polish day.

**What to build:**
- [ ] `src/cli/formatters/json.rs` — serialize `DiffResult` as clean JSON to stdout
- [ ] `src/cli/formatters/markdown.rs` — generate a markdown table:
  ```
  ## Storage Diff — ContractName
  | Variable | Type | Before | After | Status |
  |---|---|---|---|---|
  | `fee` | `uint256` | 3000 | 5000 | ⚠️ Changed |
  ```
- [ ] Wire `--output json` and `--output markdown` flags to the `diff` command
- [ ] Add a `Formatter` trait and implement it for all three formats:
  ```rust
  pub trait Formatter {
      fn format_diff(&self, diff: &DiffResult) -> String;
  }
  ```
- [ ] **Unit tests** for the diff engine — test all four statuses

**✅ Milestone:** `cargo run -- diff before.json after.json --output markdown` produces a table you could paste into a GitHub PR.

---

### 📅 Day 9 (July 18) — Storage Collision Detector

**⏱ Time: 2.5 hours**

**EVM Knowledge (20 min):**
- Read [EIP-1967](https://eips.ethereum.org/EIPS/eip-1967) — understand the standard proxy storage slots
- Read [RareSkills — Proxy Patterns](https://www.rareskills.io/post/proxy-patterns) — understand UUPS vs Transparent
- Focus on: *why* storage collisions happen during upgrades

**What to build:**
- [ ] `src/core/collision/detector.rs`:
  - `fn detect_collisions(old: &StorageLayout, new: &StorageLayout) -> Vec<Collision>`
  - Compare slot ranges: for each variable in `new`, check if it overlaps with any variable in `old` that has a different name/type
  - Handle packed slots (byte-level overlap detection within a slot)
- [ ] `src/core/collision/report.rs`:
  - Format collision report with clear ❌/✅ indicators
  - Include remediation advice ("This upgrade WILL corrupt X on mainnet")
- [ ] Wire the `check-collision` CLI command
- [ ] Exit code 1 on collision (for CI integration)
- [ ] **Unit tests** — test overlapping slots, packed slot conflicts, clean layouts

**✅ Milestone:** `cargo run -- check-collision old.json new.json` — prints ✅ or ❌ with details. Exits with code 1 on collision.

---

### 📅 Day 10 (July 19) — Config System + RPC Retry/Batching

**⏱ Time: 2.5 hours**

**🧠 LEARN FIRST (20 min):**
- [Rust Book Ch 16 — Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html) — skim it. You'll use async concurrency (tokio) not threads, but the mental model helps.
- [Tokio — select! and join!](https://tokio.rs/tokio/tutorial/select) — you'll use `tokio::join!` or `futures::future::join_all` for concurrent RPC calls

**What to build:**
- [ ] `src/config/schema.rs` — define the `.slotproberc.json` config structure with serde:
  ```rust
  struct Config {
      default_chain: Option<String>,
      rpc: HashMap<String, String>,       // chain_name -> rpc_url
      rpc_config: RpcConfig,
      artifacts_dir: Option<String>,
      snapshots_dir: Option<String>,
  }
  struct RpcConfig {
      max_concurrent: usize,
      retries: u32,
      backoff_ms: u64,
  }
  ```
- [ ] `src/config/loader.rs` — look for `.slotproberc.json` in cwd and home dir, merge with CLI flags
- [ ] Add retry logic to `reader.rs`:
  - Simple loop with exponential backoff on RPC failures
  - Configurable max retries
- [ ] Add batching: when reading multiple slots, use `tokio::join!` or `futures::future::join_all` with a concurrency semaphore (`tokio::sync::Semaphore`)
- [ ] Update `capture_snapshot` to use batched reads

**✅ Milestone:** Snapshot of a contract with 20+ variables completes in a few seconds (batched), not 20+ sequential requests. Retry handles flaky RPCs gracefully.

---

### 📅 Day 11 (July 20) — Migration Script Generator

**⏱ Time: 2.5 hours**

**🧠 LEARN FIRST (20 min):**
- [handlebars-rust docs](https://docs.rs/handlebars/latest/handlebars/) — understand `register_template_string`, `render`, custom helpers
- `cargo add handlebars` if you haven't already

**What to build:**
- [ ] Create `templates/foundry.hbs` — Handlebars template for a Foundry `forge script` migration
- [ ] Create `templates/hardhat.hbs` — Handlebars template for a Hardhat migration script
- [ ] `src/core/migration/generator.rs`:
  - `fn generate_migration(diffs, contract_name, address, format) -> Result<String>`
  - Register a `capitalize` helper for function names
  - Filter diffs to only `Changed` and `Added` entries
  - Render the template with the changes
- [ ] Wire `generate-migration` CLI command:
  - Load two snapshots → diff → generate script → write to `--out` path
- [ ] Add `--dry-run` mode: print what would be generated without writing files

**✅ Milestone:** `cargo run -- generate-migration before.json after.json --format foundry --out migrate.s.sol` produces a valid Foundry script file.

---

### 📅 Day 12 (July 21) — `--verify` Flag (Anvil Fork Verification)

**⏱ Time: 2.5 hours**

**🧠 LEARN FIRST (20 min):**
- [Rust `std::process::Command` docs](https://doc.rust-lang.org/std/process/struct.Command.html) — spawning external processes (Anvil, forge script)
- [Foundry Book — Anvil](https://book.getfoundry.sh/reference/anvil/) — `--fork-url`, `--fork-block-number`, `--port`
- [Foundry Book — forge script](https://book.getfoundry.sh/reference/forge/forge-script) — how to run a migration script

**What to build:**
- [ ] `src/core/migration/verifier.rs`:
  - `async fn verify_migration(script, before_snapshot, after_snapshot, rpc_url) -> Result<VerificationResult>`
  - Steps:
    1. Spawn `anvil --fork-url <rpc> --fork-block-number <block> --port 8546`
    2. Wait for it to be ready (check TCP connection or short sleep)
    3. Run `forge script <script_path> --rpc-url http://127.0.0.1:8546 --broadcast`
    4. Take a new snapshot against the fork
    5. Diff it against the `after` snapshot
    6. Report pass/fail
    7. Kill Anvil process in `finally`
- [ ] Wire `--verify` flag on `generate-migration` command
- [ ] Handle errors gracefully (Anvil not installed, forge not installed, script fails)

**✅ Milestone:** `cargo run -- generate-migration before.json after.json --format foundry --verify` outputs either "✅ Migration verified on Anvil fork" or a red diff showing mismatches.

---

### 📅 Day 13 (July 22) — Integration Tests + Proxy Pattern Support

**⏱ Time: 2.5 hours**

**EVM Knowledge (15 min):**
- Re-read [EIP-1967](https://eips.ethereum.org/EIPS/eip-1967) — the specific storage slots for implementation, admin, beacon
- Understand: how to detect if a contract is a proxy (read the EIP-1967 implementation slot)

**What to build:**
- [ ] Add proxy detection to snapshot:
  - Check EIP-1967 implementation slot (`0x360894...`)
  - If it's a proxy, read the implementation address and note it in the snapshot metadata
  - Support UUPS and Transparent patterns
- [ ] Write integration tests (require Anvil running):
  - `tests/integration/snapshot.rs` — fork mainnet, snapshot USDC, verify `decimals` = 6
  - `tests/integration/diff.rs` — create two snapshots at different blocks, verify diff detects changes
  - `tests/integration/collision.rs` — test with known colliding layouts
- [ ] Add test fixtures: real Foundry and Hardhat artifact JSONs in `tests/fixtures/`

**💡 Tip:** Run integration tests with: `cargo test --test integration -- --ignored` (mark them `#[ignore]` so they don't run in CI without Anvil)

**✅ Milestone:** `cargo test` — all unit tests pass. Integration tests pass against an Anvil fork.

---

### 📅 Day 14 (July 23) — Polish, GitHub Actions, README, Release Build

**⏱ Time: 2.5 hours**

Final day — no new concepts. Focus on shipping a usable v0.1.

**What to build:**
- [ ] Add `indicatif` progress bars/spinners to long operations (snapshot, verify)
- [ ] Add helpful error messages everywhere (`anyhow::Context` — `.context("Failed to parse artifact")`)
- [ ] Create `.github/workflows/slotprobe-check.yml` — a ready-made GitHub Actions workflow file users can copy
- [ ] Create `.slotproberc.example.json`
- [ ] Write a solid `README.md` with:
  - Installation (`cargo install --path .`)
  - Quick start examples for each command
  - Configuration reference
- [ ] `cargo build --release` — verify the binary works standalone
- [ ] `cargo clippy --all-targets -- -D warnings` — fix all warnings
- [ ] `cargo fmt` — format everything
- [ ] Run the full flow end-to-end one more time:
  ```bash
  ./target/release/slotprobe snapshot 0x... --chain mainnet --artifact ./artifact.json --out before.json
  # (simulate an upgrade / use different block)
  ./target/release/slotprobe snapshot 0x... --chain mainnet --artifact ./artifact.json --block 19001000 --out after.json
  ./target/release/slotprobe diff before.json after.json
  ./target/release/slotprobe diff before.json after.json --output markdown
  ./target/release/slotprobe check-collision old_artifact.json new_artifact.json
  ./target/release/slotprobe generate-migration before.json after.json --format foundry --out migrate.s.sol
  ```

**✅ Milestone:** A working, polished `slotprobe` binary. Clean code, tests pass, README exists. v0.1 is done. 🎉

---

## 📊 Daily Summary Table

| Day | Date | What You Build | New Rust/Tools to Learn | Hours |
|-----|------|---------------|------------------------|-------|
| 1 | Jul 10 | Project setup + CLI skeleton | Ch 14 (Cargo), Clap derive, Serde derive | 2.5 |
| 2 | Jul 11 | Artifact parser types + Foundry parser | **Ch 13 (Closures & Iterators)**, Serde attributes | 2.5 |
| 3 | Jul 12 | RPC client + slot reader | **Async/Await (Tokio tutorial)**, Alloy basics | 2.5 |
| 4 | Jul 13 | Slot calculator + type decoder | EVM storage layout deep dive (no new Rust) | 2.5 |
| 5 | Jul 14 | Snapshot capture + serialization | Combining async + serde (practice day) | 2.5 |
| 6 | Jul 15 | Filters, dry-run, Hardhat parser | No new concepts (consolidation day) | 2.5 |
| 7 | Jul 16 | Diff engine + terminal output | **Ch 15 (Smart Pointers)**, Ch 17.2 (Trait Objects) | 2.5 |
| 8 | Jul 17 | JSON + Markdown formatters | No new concepts (polish day) | 2.5 |
| 9 | Jul 18 | Collision detector | EIP-1967, proxy patterns | 2.5 |
| 10 | Jul 19 | Config system + RPC batching | **Ch 16 (Concurrency)**, Tokio join!/Semaphore | 2.5 |
| 11 | Jul 20 | Migration script generator | Handlebars templates | 2.5 |
| 12 | Jul 21 | `--verify` flag (Anvil fork) | `std::process::Command`, Foundry scripting | 2.5 |
| 13 | Jul 22 | Integration tests + proxy support | EIP-1967 deep dive | 2.5 |
| 14 | Jul 23 | Polish, README, release build | No new concepts (ship day) | 2.5 |

---

## 🧠 Learning Strategy

The plan is structured so that **you never hit a wall**. Every day that introduces a new Rust concept gives you 20–45 minutes of targeted reading *before* you start coding. Here's the reading order:

```
Day 1  → Ch 14 (Cargo)           — 15 min
Day 2  → Ch 13 (Closures/Iters)  — 30 min ← MOST IMPORTANT
Day 3  → Tokio Tutorial           — 45 min ← HARDEST DAY
Day 7  → Ch 15 (Smart Pointers)   — 20 min
Day 7  → Ch 17.2 (Trait Objects)  — 10 min
Day 10 → Ch 16 (Concurrency)      — 20 min (skim)
```

**Days 3 and 2 are the hardest learning days.** If you fall behind, it's okay to split Day 3 into two sessions. Async is the single biggest new concept you'll encounter.

---

## ⚠️ Risks & Contingency

| Risk | Mitigation |
|------|-----------|
| Async/Tokio takes longer to grok | Day 3 has buffer — you can spill into Day 4's EVM work (pure logic, no new Rust) |
| Alloy API changes or confusing docs | Fall back to raw `reqwest` HTTP calls to JSON-RPC endpoints. It's just POST requests. |
| Anvil/Foundry not installed | Days 12-13 need Foundry. Install it on Day 1: `curl -L https://foundry.paradigm.xyz \| bash` |
| Running over 2.5 hours | Days 6, 8, 14 are lighter "consolidation" days. Use them as overflow. |
| Don't have an RPC key | Sign up for [Alchemy free tier](https://www.alchemy.com/) on Day 1. |

---

## 🏁 What You'll Have After 2 Weeks

A working `slotprobe` CLI binary that can:
1. **Snapshot** a live contract's storage state with named variables
2. **Diff** two snapshots with colored terminal, JSON, and Markdown output
3. **Detect storage collisions** between two contract versions
4. **Generate migration scripts** for Foundry and Hardhat
5. **Auto-verify** migrations on an Anvil fork
6. **Handle proxy patterns** (EIP-1967, UUPS, Transparent)
7. **Batch RPC calls** with retry and rate limiting
8. **Run in CI** via GitHub Actions

And you'll have learned:
- Closures, iterators, smart pointers, trait objects, async/await in Rust
- EVM storage layout at a deep level
- How to build and distribute a real CLI tool in Rust
- How to work with external processes (Anvil, forge)

**This is a portfolio-grade project.** Ship it. 🚀
