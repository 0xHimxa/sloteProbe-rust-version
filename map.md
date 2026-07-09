# 🔬 SlotProbe

> **The missing developer tool for Web3** — snapshot, diff, and migrate smart contract state across upgrades and chains. No more manual slot hunting. No more blind upgrades.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange)](https://www.rust-lang.org/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)
[![Built for EVM](https://img.shields.io/badge/Built%20for-EVM-purple)](https://ethereum.org)

---

## 📖 Table of Contents

- [The Problem](#-the-problem)
- [Why SlotProbe Doesn't Exist Yet](#-why-SlotProbe-doesnt-exist-yet)
- [What SlotProbe Does](#-what-SlotProbe-does)
- [Key Features](#-key-features)
- [How It Works (Architecture)](#-how-it-works-architecture)
- [Tech Stack](#-tech-stack)
- [Skills You Need to Build This](#-skills-you-need-to-build-this)
- [Project Structure](#-project-structure)
- [Step-by-Step Build Guide](#-step-by-step-build-guide)
  - [Phase 1 — EVM Storage Engine](#phase-1--evm-storage-engine)
  - [Phase 2 — Artifact Parser](#phase-2--artifact-parser)
  - [Phase 3 — Snapshot System](#phase-3--snapshot-system)
  - [Phase 4 — Diff Engine](#phase-4--diff-engine)
  - [Phase 5 — Migration Generator](#phase-5--migration-generator)
  - [Phase 6 — Multi-Chain Consistency Checker](#phase-6--multi-chain-consistency-checker)
  - [Phase 7 — CLI Interface](#phase-7--cli-interface)
  - [Phase 8 — Foundry & Hardhat Integration](#phase-8--foundry--hardhat-integration)
- [Standout Features to Add](#-standout-features-to-add)
- [Testing Strategy](#-testing-strategy)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)

---

## 🔥 The Problem

Every serious smart contract protocol eventually faces these painful situations:

**Situation 1 — Upgrading a Proxy Contract**

You've deployed a UUPS upgradeable proxy. You write a new implementation. You run `forge upgrade`. *But did your new implementation accidentally overwrite storage slot 3 — the one your old contract used for `_owner`?*

There's no tool that will tell you this clearly, before the upgrade corrupts live state on mainnet. Slither has a basic collision check, but it gives you raw slot numbers and no workflow. You end up manually comparing compiler output, slot by slot, hoping you don't miss something. Protocols have lost **millions of dollars** to storage collisions in upgradeable contracts.

**Situation 2 — Deploying Across Multiple Chains**

Your protocol is live on Ethereum, Arbitrum, Base, and Optimism. Governance passed a parameter update on Ethereum three weeks ago. *Did that change get applied to the Arbitrum deployment?* There is currently no tool that lets you query the same contract across chains, compare their state, and flag divergences. Teams do this manually with spreadsheets. Or they don't do it at all.

**Situation 3 — Post-Upgrade State Audit**

You upgraded a contract. You *think* the migration went fine. But did all the old state migrate correctly? Are there mappings with stale values? Did any storage variable move slots between versions?

You currently have no way to take a snapshot *before* and *after* an upgrade and semantically diff them at the variable name level — not the raw hex level.

**The result:** Protocol engineers waste days on what should be a 5-minute CLI command. And sometimes they make mistakes that cost millions.

---

## 🔍 Why SlotProbe Doesn't Exist Yet

Before building this, we verified the landscape of existing tools:

| Tool | What it Does | What it Misses |
|---|---|---|
| `sol2uml` | Visualizes storage layout as diagrams | No live state, no diffs, no migration |
| `hardhat-storage-layout` | Prints slot table from compiled artifacts | No live values, no diff, no cross-chain |
| `slither-read-storage` | Reads raw storage slots from a live contract | No variable name mapping, no diff, no migration |
| `SmartMuv` | Extracts storage state for migration | No semantic diff, no cross-chain checker, no script generation |
| Tenderly | Monitors live contract state | No upgrade diffs, no migration tooling, no offline workflow |
| OpenZeppelin Upgrades | Validates upgrade compatibility | Limited to layout collision check, no live state diff |

**SlotProbe is the first tool to combine all five capabilities into one workflow:**
snapshot → semantic diff → collision detection → migration script generation → cross-chain consistency check.

---

## ✅ What SlotProbe Does

```bash
# Take a snapshot of a contract's state before an upgrade
slotprobe snapshot 0xUniswapV3Pool --chain mainnet --block 19000000 --out before.json

# Take a snapshot after the upgrade
slotprobe snapshot 0xUniswapV3Pool --chain mainnet --block 19001000 --out after.json

# Diff the two snapshots — by variable name, not by raw slot
slotprobe diff before.json after.json

# Check if a storage collision would occur in a proposed upgrade
slotprobe check-collision ./old/MyContract.json ./new/MyContract.json

# Verify that the same contract has consistent state across chains
slotprobe cross-chain 0xUniswapV3Pool --chains mainnet,arbitrum,base --vars fee,tickSpacing

# Auto-generate a Foundry migration script from a diff
slotprobe generate-migration before.json after.json --format foundry --out migrate.s.sol
```

---

## ⚡ Key Features

### 1. Semantic State Snapshots

SlotProbe reads raw EVM storage slots via `eth_getStorageAt` and maps them back to **named Solidity variables** using the compiler's `storageLayout` output. Instead of seeing `slot 3 = 0x000...0001`, you see `_owner = 0xab12...`.

Supports:
- Simple types (uint, address, bool, bytes32)
- Packed slots (multiple variables in one 32-byte slot)
- Dynamic arrays (reads length + elements)
- Mappings (with user-supplied key sets)
- Nested structs
- All proxy patterns (EIP-1967, Transparent, UUPS, Diamond/EIP-2535)

### 2. Human-Readable Diff Engine

Like `git diff`, but for contract storage. SlotProbe diffs two snapshots and outputs:

```diff
Contract: UniswapV3Pool (0xabc...123)
Block: 19000000 → 19001000

  fee: 3000 (unchanged)
- tickSpacing: 60
+ tickSpacing: 10
- _owner: 0xDead...Beef
+ _owner: 0xAlice...1234
  liquidity: 4823947239847 (unchanged)
```

### 3. Storage Collision Detector

Compares two contract versions' compiled storage layouts and flags any slot conflicts **before** deployment. Goes beyond OpenZeppelin's basic check — handles Diamond proxies, inheritance chains, and packed slot edge cases.

### 4. Cross-Chain Consistency Checker

Point SlotProbe at the same contract address across multiple chains. It will read the same set of state variables on each chain and produce a report of what is in sync and what has diverged.

### 5. Migration Script Generator

Given a diff, SlotProbe generates a ready-to-run Foundry or Hardhat migration script. The generated script handles:
- Setting changed values on the new contract
- Migrating dynamic array contents
- Batch operations to minimize gas
- Verification checks post-migration

### 6. CI/CD Integration

SlotProbe can run in CI pipelines — fail the build if a proposed upgrade would cause storage collisions, or if cross-chain state has drifted beyond acceptable thresholds. The compiled Rust binary makes this trivial to drop into any CI runner with zero runtime dependencies.

---

## 🏗 How It Works (Architecture)

```
┌─────────────────────────────────────────────────────────────────┐
│                        SlotProbe CLI                             │
│         snapshot | diff | check-collision | cross-chain          │
│                    generate-migration                            │
└──────────────────────────┬──────────────────────────────────────┘
                           │
          ┌────────────────┼─────────────────┐
          ▼                ▼                 ▼
┌─────────────────┐ ┌──────────────┐ ┌─────────────────┐
│  Artifact       │ │  EVM Storage │ │  Multi-chain    │
│  Parser         │ │  Engine      │ │  RPC Manager    │
│                 │ │              │ │                 │
│ Foundry JSON    │ │ eth_getStorAt│ │ alloy providers │
│ Hardhat JSON    │ │ Slot decoder │ │ per chain       │
│ storageLayout   │ │ Type mapper  │ │                 │
└────────┬────────┘ └──────┬───────┘ └────────┬────────┘
         │                 │                  │
         └─────────────────▼──────────────────┘
                           │
                  ┌────────▼────────┐
                  │  Snapshot Store │
                  │  (JSON files)   │
                  └────────┬────────┘
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
     ┌──────────────┐ ┌─────────┐ ┌──────────────────┐
     │ Diff Engine  │ │Collision│ │Migration Script  │
     │              │ │Detector │ │Generator         │
     │ Semantic     │ │         │ │                  │
     │ variable-    │ │Slot     │ │Handlebars        │
     │ level diff   │ │overlap  │ │Templates         │
     │              │ │checker  │ │(Foundry/Hardhat) │
     └──────┬───────┘ └────┬────┘ └────────┬─────────┘
            │              │               │
            └──────────────▼───────────────┘
                           │
                  ┌────────▼────────┐
                  │  Output Layer   │
                  │  Terminal / JSON│
                  │  / HTML Report  │
                  └─────────────────┘
```

---

## 🛠 Tech Stack

### Core Language

**Rust (edition 2021, 1.78+)** — Performance, memory safety, and a single static binary matter a lot for a CLI tool that needs to run fast in CI and handle large storage layouts without a runtime. Rust's strong type system is a great match for the strict, well-defined shapes of `storageLayout` JSON and EVM word encoding.

### Blockchain Interaction

| Crate | Version | Purpose |
|---|---|---|
| `alloy` | `^0.9` | Primary RPC client and EVM primitives (successor to `ethers-rs`, actively maintained by Paradigm). Used for all `eth_getStorageAt` calls, address/`U256`/`B256` types, and keccak hashing |
| `ethers-rs` | `^2.x` | Optional compatibility layer if you need to interop with older tooling or examples that still reference it |

### EVM & Solidity

| Tool | Purpose |
|---|---|
| `solc` compiler output | The `storageLayout` JSON field tells you exactly which slot each variable occupies. This is your foundation |
| Foundry build artifacts | `out/ContractName.sol/ContractName.json` — contains ABI + full storage layout |
| Hardhat artifacts | `artifacts/contracts/...` — slightly different format, same data |

### CLI Framework

| Crate | Purpose |
|---|---|
| `clap` (derive API) | Subcommand routing (`snapshot`, `diff`, `check-collision`, etc.) with compile-time-checked argument parsing |
| `indicatif` | Terminal spinners and progress bars while waiting for RPC responses |
| `owo-colors` (or `colored`) | Colored terminal output for diffs (red = removed, green = added) |
| `comfy-table` | Tabular output for storage layout and diff reports |
| `dialoguer` | Interactive prompts when config is missing |

### Data Processing

| Crate | Purpose |
|---|---|
| `serde` / `serde_json` | Struct (de)serialization for config files, artifacts, and snapshots |
| `schemars` + manual validation, or `validator` | Config file schema validation (`.slotproberc.json`) |
| `similar` | Base diffing primitive — you'll extend it with your own semantic-diff layer on top |
| `handlebars` (`handlebars-rust`) | Template engine for migration script code generation |
| `alloy-primitives::keccak256` | Computing mapping and dynamic array slot positions |

### Async Runtime

| Crate | Purpose |
|---|---|
| `tokio` | Async runtime powering all RPC calls, including concurrent multi-chain queries via `tokio::join!` / `futures::future::join_all` |

### Testing

| Tool | Purpose |
|---|---|
| Built-in `cargo test` + `#[tokio::test]` | Fast, native async test runner |
| `rstest` | Table-driven / parameterized tests (handy for decoder and slot-calculator test matrices) |
| `anvil` (Foundry) | Local EVM fork. You fork mainnet and run integration tests against real contracts, driven from Rust via `alloy`'s Anvil test helpers |
| `mockito` or `wiremock` | Mock RPC responses for unit tests |

### Build & Distribution

| Tool | Purpose |
|---|---|
| `cargo build --release` | Produces a single static optimized binary — no runtime, no `node_modules` |
| `cargo install --path .` | Local install for development |
| `cross` | Cross-compilation for distributing prebuilt binaries (macOS/Linux/Windows, x86_64/arm64) |
| `cargo-release` / `release-plz` | Version management and changelog generation |
| `cargo publish` | Publish the library portion of the crate to crates.io for programmatic use |

---

## 📚 Skills You Need to Build This

This is a hard project. Here is an honest breakdown of every skill you need, what level you need it at, and exactly where to learn it.

---

### 1. Rust (Required — Intermediate Level)

You need to be comfortable with ownership and borrowing, traits, generics, `Result`/`Option` and the `?` operator, and `async`/`await` with Tokio. You don't need to be an expert, but you need to be past beginner — this project uses trait objects, custom error types, and non-trivial lifetime-free async code.

**Where to learn:**
- [The Rust Book](https://doc.rust-lang.org/book/) — official, free, comprehensive. Start here, especially chapters on ownership, traits, and error handling.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/) — free, practical, runnable snippets for every concept.
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) — official async runtime guide. Essential once you start making concurrent RPC calls.
- [Jon Gjengset's YouTube channel](https://www.youtube.com/@jonhoo) — advanced, in-depth Rust walkthroughs (traits, async internals, crate design).

**What to focus on:** Ownership/borrowing, `Result`/`?`, trait objects vs generics, `serde` derive macros, `async`/`await` with Tokio, and basic `unsafe`-free FFI-free crate design.

---

### 2. EVM Storage Layout (Required — This Is the Core Skill)

This is the hardest and most important thing to learn for this project. You need to understand how Solidity stores variables in 32-byte slots, how mappings compute their slot using keccak256, how dynamic arrays work, how packed storage works, and how the `solc` compiler exposes this via the `storageLayout` output.

**Where to learn:**
- [Solidity Docs — Layout of State Variables in Storage](https://docs.soliditylang.org/en/latest/internals/layout_in_storage.html) — read this until you fully understand it. This is your bible.
- [Trail of Bits Blog — Shedding Smart Contract Storage with Slither](https://blog.trailofbits.com/2022/07/28/shedding-smart-contract-storage-with-slither/) — excellent practical walkthrough.
- [OpenZeppelin Blog — Storage Gaps](https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps) — explains why storage layout matters for upgrades.
- [EVM Codes](https://www.evm.codes/) — reference for every EVM opcode. You'll use this to understand SLOAD/SSTORE.
- [Noxx's EVM Deep Dives on Mirror.xyz](https://noxx.substack.com/) — some of the best EVM internals writing available, free.

**What to practice:** Manually calculate the slot for a mapping key. Manually figure out where a struct's second field lives. Then verify with `slither-read-storage` or `cast storage`.

---

### 3. Foundry (Required — Beginner to Intermediate)

Foundry is the modern smart contract development framework. You'll use Anvil (its local EVM) for integration tests, and you need to understand its artifact format to parse `storageLayout` from build output. Foundry itself stays a separate toolchain (written in Rust!) that your Rust CLI shells out to or forks against.

**Where to learn:**
- [Foundry Book](https://book.getfoundry.sh/) — the official documentation. Free. Work through the entire Getting Started section and the Testing chapter.
- [Patrick Collins — Learn Foundry YouTube Series](https://www.youtube.com/watch?v=umepbfKp5rI) — free, comprehensive, beginner-friendly video course.

**What to practice:** `forge build` and inspect the `out/` folder JSON. Run `cast storage <address> <slot>` against a fork to manually verify slot values.

---

### 4. Alloy (Required — Beginner)

`alloy` is your primary RPC client and EVM primitives crate, replacing what `viem`/`ethers.js` would do in a TypeScript stack. It's actively developed, well-typed, and you'll pick it up fast if you're comfortable with Rust generics and async.

**Where to learn:**
- [Alloy Book](https://alloy.rs/) — start with the "Getting Started" section, then read the provider and `get_storage_at` reference carefully.
- [Alloy GitHub examples](https://github.com/alloy-rs/examples) — runnable example projects covering providers, contract calls, and typed primitives.

---

### 5. Rust CLI Development (Required — Beginner)

Building a CLI is more involved than building a library. You need to understand argument parsing with `clap`'s derive macros, process exit codes, stdin/stdout, and how to distribute a compiled binary.

**Where to learn:**
- [Clap documentation](https://docs.rs/clap/latest/clap/) — the derive-macro guide alone teaches you 90% of what you need.
- [Command Line Applications in Rust ("CLI Book")](https://rust-cli.github.io/book/) — free official-adjacent guide, practical walkthrough covering argument parsing, error handling, testing, and packaging.
- [crates.io — Publishing Guide](https://doc.rust-lang.org/cargo/reference/publishing.html) — for when you're ready to publish the library crate.

---

### 6. Solidity Basics (Required — Beginner)

You don't need to be a Solidity developer, but you need to read Solidity code fluently. You need to understand inheritance, proxy patterns (what UUPS vs Transparent means), and how `delegatecall` works.

**Where to learn:**
- [CryptoZombies](https://cryptozombies.io/) — free, gamified, beginner-friendly introduction to Solidity.
- [Solidity by Example](https://solidity-by-example.org/) — short practical code snippets covering every concept. Free.
- [OpenZeppelin Contracts — Upgrades Plugin Docs](https://docs.openzeppelin.com/upgrades-plugins/1.x/) — read the proxy patterns section specifically.

---

### 7. Proxy Patterns (Required — Intermediate)

Understanding EIP-1967, Transparent Proxy, UUPS, and EIP-2535 Diamond is non-negotiable. Your tool needs to handle all of them because real protocols use all of them.

**Where to learn:**
- [OpenZeppelin — Proxy Upgrade Pattern](https://docs.openzeppelin.com/contracts/4.x/api/proxy) — official documentation with clear diagrams.
- [EIP-1967](https://eips.ethereum.org/EIPS/eip-1967) — the actual standard. Read it.
- [EIP-2535 Diamond Standard](https://eips.ethereum.org/EIPS/eip-2535) — the Diamond proxy pattern. Complex but well-documented.
- [Rareskills — Proxy Patterns](https://www.rareskills.io/post/proxy-patterns) — free, excellent breakdown of all patterns with diagrams.

---

### 8. Code Generation with Templates (Nice to Have — Beginner)

For the migration script generator, you'll use `handlebars-rust` to generate Solidity/Foundry code from templates.

**Where to learn:**
- [handlebars-rust docs.rs page](https://docs.rs/handlebars/latest/handlebars/) — covers registering templates, helpers, and rendering. Free.

---

### 9. Testing with Cargo + Anvil (Required — Beginner)

Your integration tests will fork mainnet using Anvil, then run your tool against real contracts with known storage layouts, all driven through native `cargo test`.

**Where to learn:**
- [The Rust Book — Testing chapter](https://doc.rust-lang.org/book/ch11-00-testing.html) — official guide to `#[test]`, `#[tokio::test]`, and organizing unit vs. integration tests.
- [Foundry Book — Anvil](https://book.getfoundry.sh/anvil/) — specifically the `--fork-url` flag for mainnet forking.
- [rstest docs.rs page](https://docs.rs/rstest/latest/rstest/) — for parameterized/table-driven tests.

---

## 📁 Project Structure

```
SlotProbe/
├── src/
│   ├── main.rs                   # Binary entry point (Clap setup, tokio runtime)
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── snapshot.rs       # `slotprobe snapshot` command
│   │   │   ├── diff.rs           # `slotprobe diff` command
│   │   │   ├── check_collision.rs
│   │   │   ├── cross_chain.rs
│   │   │   └── generate_migration.rs
│   │   └── formatters/
│   │       ├── mod.rs
│   │       ├── terminal.rs       # owo-colors + comfy-table output
│   │       ├── json.rs           # JSON output
│   │       └── html.rs           # HTML report output
│   │
│   ├── core/
│   │   ├── mod.rs
│   │   ├── storage_engine/
│   │   │   ├── mod.rs
│   │   │   ├── reader.rs         # eth_getStorageAt wrapper (alloy provider)
│   │   │   ├── slot_calculator.rs # keccak256 slot math for mappings/arrays
│   │   │   ├── decoder.rs        # Raw hex → typed Solidity value
│   │   │   └── packed.rs         # Packed slot handling
│   │   │
│   │   ├── artifact_parser/
│   │   │   ├── mod.rs
│   │   │   ├── foundry.rs        # Parse Foundry out/ JSON artifacts
│   │   │   ├── hardhat.rs        # Parse Hardhat artifacts/
│   │   │   ├── types.rs          # Shared StorageLayout struct definitions
│   │   │   └── normalizer.rs     # Normalize both formats to common schema
│   │   │
│   │   ├── snapshot/
│   │   │   ├── mod.rs
│   │   │   ├── capture.rs        # Orchestrates a full snapshot
│   │   │   ├── store.rs          # Read/write snapshot JSON files
│   │   │   └── types.rs          # Snapshot schema types
│   │   │
│   │   ├── diff/
│   │   │   ├── mod.rs
│   │   │   ├── engine.rs         # Core diff logic
│   │   │   ├── semantic.rs       # Variable-name-level diff (not raw slots)
│   │   │   └── types.rs          # DiffResult types
│   │   │
│   │   ├── collision/
│   │   │   ├── mod.rs
│   │   │   ├── detector.rs       # Storage slot collision detection
│   │   │   ├── proxy_handler.rs  # Per-proxy-pattern collision logic
│   │   │   └── report.rs         # Collision report formatting
│   │   │
│   │   ├── cross_chain/
│   │   │   ├── mod.rs
│   │   │   ├── checker.rs        # Queries same vars across multiple chains
│   │   │   └── consistency.rs    # Generates consistency report
│   │   │
│   │   └── migration/
│   │       ├── mod.rs
│   │       ├── generator.rs      # Generates migration scripts from diffs
│   │       └── templates/
│   │           ├── foundry.hbs   # Foundry script template
│   │           └── hardhat.hbs   # Hardhat script template
│   │
│   ├── rpc/
│   │   ├── mod.rs
│   │   ├── client.rs             # alloy provider factory per chain
│   │   ├── chains.rs             # Chain configs (mainnet, arbitrum, base, etc.)
│   │   └── retry.rs              # RPC retry logic with exponential backoff
│   │
│   └── config/
│       ├── mod.rs
│       ├── loader.rs             # Load .slotproberc.json
│       └── schema.rs             # serde-derived config schema
│
├── tests/
│   ├── slot_calculator.rs        # Unit tests: verify known keccak256 outputs
│   ├── decoder.rs                # Unit tests: verify each Solidity type decodes
│   ├── artifact_parser.rs        # Unit tests: fixture JSON files (Foundry + Hardhat)
│   ├── diff_engine.rs            # Unit tests: all four diff statuses
│   └── integration/
│       ├── snapshot.rs           # Forks mainnet, snapshots real contracts
│       ├── diff.rs
│       └── collision.rs
│
├── templates/                    # Handlebars migration templates
├── .slotproberc.example.json     # Example config file
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## 🪜 Step-by-Step Build Guide

Follow these phases in order. Each phase produces something testable before you move to the next.

---

### Phase 1 — EVM Storage Engine

**Goal:** Given a contract address, chain RPC URL, and slot number, return the decoded value at that slot.

**What to build:** `src/core/storage_engine/`

**Step 1.1 — Set up the project**

```bash
cargo new slotprobe --bin
cd slotprobe
cargo add tokio --features full
cargo add alloy --features "full"
cargo add serde --features derive
cargo add serde_json
cargo add clap --features derive
cargo add indicatif owo-colors comfy-table dialoguer
cargo add handlebars
cargo add anyhow thiserror
```

`Cargo.toml` (relevant excerpt):

```toml
[package]
name = "slotprobe"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "slotprobe"
path = "src/main.rs"

[dependencies]
tokio = { version = "1", features = ["full"] }
alloy = { version = "0.9", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
clap = { version = "4", features = ["derive"] }
indicatif = "0.17"
owo-colors = "4"
comfy-table = "7"
dialoguer = "0.11"
handlebars = "6"
anyhow = "1"
thiserror = "2"
```

**Step 1.2 — Build the RPC client factory**

```rust
// src/rpc/client.rs
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::transports::http::{Client, Http};
use anyhow::Result;

pub fn get_client(rpc_url: &str) -> Result<RootProvider<Http<Client>>> {
    let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);
    Ok(provider)
}
```

**Step 1.3 — Build the raw slot reader**

```rust
// src/core/storage_engine/reader.rs
use alloy::primitives::{Address, B256, U256};
use alloy::providers::Provider;
use anyhow::Result;

use crate::rpc::client::get_client;

pub async fn read_slot(
    address: Address,
    slot: U256,
    rpc_url: &str,
    block_number: Option<u64>,
) -> Result<B256> {
    let provider = get_client(rpc_url)?;

    let value = if let Some(block) = block_number {
        provider
            .get_storage_at(address, slot)
            .block_id(block.into())
            .await?
    } else {
        provider.get_storage_at(address, slot).await?
    };

    Ok(B256::from(value))
}
```

**Step 1.4 — Build the slot calculator for mappings and dynamic arrays**

This is the most complex part of Phase 1. Study the Solidity storage docs first.

```rust
// src/core/storage_engine/slot_calculator.rs
use alloy::primitives::{keccak256, Address, B256, U256};

/// For a mapping(address => uint256) at base slot N:
/// The slot for key K is keccak256(abi.encode(K, N))
pub fn mapping_slot(key: Address, base_slot: U256) -> U256 {
    let mut buf = [0u8; 64];
    buf[12..32].copy_from_slice(key.as_slice()); // left-pad address into a bytes32
    buf[32..64].copy_from_slice(&base_slot.to_be_bytes::<32>());
    let hash: B256 = keccak256(buf);
    U256::from_be_bytes(hash.0)
}

/// For a dynamic array at base slot N:
/// Length is at slot N. Element i is at keccak256(N) + i
pub fn array_element_slot(base_slot: U256, index: U256) -> U256 {
    let hash: B256 = keccak256(base_slot.to_be_bytes::<32>());
    U256::from_be_bytes(hash.0) + index
}
```

**Step 1.5 — Build the type decoder**

Maps raw 32-byte values to typed Solidity values based on the type string from `storageLayout`.

```rust
// src/core/storage_engine/decoder.rs
use alloy::primitives::{Address, B256, U256};
use serde_json::Value;

pub fn decode_value(raw: B256, ty: &str) -> Value {
    let value = U256::from_be_bytes(raw.0);

    if ty == "bool" {
        return Value::Bool(value != U256::ZERO);
    }

    if ty.starts_with("uint") {
        return Value::String(value.to_string());
    }

    if ty.starts_with("int") {
        let bits: u32 = ty.trim_start_matches("int").parse().unwrap_or(256);
        let max = U256::from(1u8) << (bits - 1);
        let signed = if value >= max {
            // two's complement negative value
            let modulus = U256::from(1u8) << bits;
            format!("-{}", modulus - value)
        } else {
            value.to_string()
        };
        return Value::String(signed);
    }

    if ty == "address" {
        let addr = Address::from_slice(&raw.0[12..32]);
        return Value::String(format!("{addr:#x}"));
    }

    if ty.starts_with("bytes") {
        return Value::String(format!("{raw:#x}"));
    }

    // fallback for complex types (structs, mappings, arrays handled elsewhere)
    Value::String(format!("{raw:#x}"))
}
```

**Milestone:** You can call `read_slot(address, U256::ZERO, rpc_url, None).await` and get back a decoded value. Test this against a known contract on a mainnet fork using Anvil.

---

### Phase 2 — Artifact Parser

**Goal:** Given a Foundry or Hardhat build artifact, extract a normalized `StorageLayout` object that maps each variable to its slot, offset, type, and size.

**What to build:** `src/core/artifact_parser/`

**Step 2.1 — Define the normalized schema**

```rust
// src/core/artifact_parser/types.rs
use alloy::primitives::U256;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageVariable {
    pub name: String,
    pub r#type: String,
    #[serde(with = "u256_string")]
    pub slot: U256,
    pub offset: u32,          // byte offset within the slot (for packed vars)
    pub number_of_bytes: u32,
    pub label: String,        // human-readable type (e.g. "mapping(address => uint256)")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLayout {
    pub contract_name: String,
    pub variables: Vec<StorageVariable>,
    pub types: HashMap<String, TypeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    pub encoding: String, // "inplace" | "mapping" | "dynamic_array" | "bytes"
    pub number_of_bytes: u32,
    pub members: Option<Vec<StorageVariable>>, // for structs
    pub key: Option<String>,                   // for mappings
    pub value: Option<String>,                 // for mappings
    pub base: Option<String>,                  // for arrays
}

// U256 doesn't serialize to JSON directly in a friendly way, so we
// round-trip it through a decimal string.
mod u256_string {
    use alloy::primitives::U256;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
```

**Step 2.2 — Parse Foundry artifacts**

Foundry outputs artifacts to `out/ContractName.sol/ContractName.json`. The storage layout is under the `storageLayout` key.

```rust
// src/core/artifact_parser/foundry.rs
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

use super::types::{StorageLayout, StorageVariable};

pub fn parse_foundry_artifact(path: &Path) -> Result<StorageLayout> {
    let raw: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let layout = &raw["storageLayout"];

    let variables = layout["storage"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|v| {
            let ty = v["type"].as_str().unwrap_or_default().to_string();
            let number_of_bytes = layout["types"][&ty]["numberOfBytes"]
                .as_str()
                .unwrap_or("32")
                .parse()
                .unwrap_or(32);

            StorageVariable {
                name: v["label"].as_str().unwrap_or_default().to_string(),
                r#type: ty,
                slot: v["slot"].as_str().unwrap_or("0").parse().unwrap_or_default(),
                offset: v["offset"].as_u64().unwrap_or(0) as u32,
                number_of_bytes,
                label: v["label"].as_str().unwrap_or_default().to_string(),
            }
        })
        .collect();

    Ok(StorageLayout {
        contract_name: raw["contractName"].as_str().unwrap_or_default().to_string(),
        variables,
        types: serde_json::from_value(layout["types"].clone())?,
    })
}
```

**Step 2.3 — Parse Hardhat artifacts**

Hardhat stores artifacts in `artifacts/contracts/.../<ContractName>.json` but storage layout requires enabling `outputSelection` in `hardhat.config.ts`. Document this requirement clearly for users. The parsing logic mirrors `foundry.rs` closely, since both ultimately expose the same `solc` `storageLayout` shape — the main difference is where the JSON lives on disk and a couple of top-level key names.

**Milestone:** You can pass any Foundry or Hardhat artifact file path to your parser and get back a normalized `StorageLayout` with named variables and their slots.

---

### Phase 3 — Snapshot System

**Goal:** Combine the storage engine and artifact parser to capture a full semantic snapshot of a live contract's state.

**What to build:** `src/core/snapshot/`

**Step 3.1 — Build the snapshot capture logic**

```rust
// src/core/snapshot/capture.rs
use alloy::primitives::{Address, U256};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::artifact_parser::types::StorageLayout;
use crate::core::storage_engine::decoder::decode_value;
use crate::core::storage_engine::reader::read_slot;

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotEntry {
    pub name: String,
    pub r#type: String,
    pub slot: String, // stored as decimal string, see types.rs pattern
    pub raw_value: String,
    pub decoded_value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub address: String,
    pub chain: String,
    pub block_number: u64,
    pub timestamp: i64,
    pub contract_name: String,
    pub state: Vec<SnapshotEntry>,
}

pub async fn capture_snapshot(
    address: Address,
    layout: &StorageLayout,
    chain: &str,
    rpc_url: &str,
    block_number: Option<u64>,
) -> Result<Snapshot> {
    let mut entries = Vec::with_capacity(layout.variables.len());

    for variable in &layout.variables {
        let raw = read_slot(address, variable.slot, rpc_url, block_number).await?;
        entries.push(SnapshotEntry {
            name: variable.name.clone(),
            r#type: variable.r#type.clone(),
            slot: variable.slot.to_string(),
            raw_value: format!("{raw:#x}"),
            decoded_value: decode_value(raw, &variable.r#type),
        });
    }

    Ok(Snapshot {
        address: format!("{address:#x}"),
        chain: chain.to_string(),
        block_number: block_number.unwrap_or(0),
        timestamp: chrono::Utc::now().timestamp(),
        contract_name: layout.contract_name.clone(),
        state: entries,
    })
}
```

**Step 3.2 — Serialization**

Because `U256`/slot numbers and hashes round-trip cleanly through `serde_json` as strings (no special "bigint" handling needed like in JS), saving and loading is just plain `serde_json`:

```rust
// src/core/snapshot/store.rs
use anyhow::Result;
use std::fs;
use std::path::Path;

use super::capture::Snapshot;

pub fn save_snapshot(snapshot: &Snapshot, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(snapshot)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_snapshot(path: &Path) -> Result<Snapshot> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}
```

**Milestone:** Run `slotprobe snapshot` against a real contract on a mainnet fork and produce a human-readable JSON file showing all named variables and their values.

---

### Phase 4 — Diff Engine

**Goal:** Given two snapshots, produce a semantic diff at the variable name level.

**What to build:** `src/core/diff/`

**Step 4.1 — Core diff logic**

```rust
// src/core/diff/engine.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::core::snapshot::capture::Snapshot;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiffStatus {
    Added,
    Removed,
    Changed,
    Unchanged,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffEntry {
    pub name: String,
    pub r#type: String,
    pub status: DiffStatus,
    pub before: Option<Value>,
    pub after: Option<Value>,
}

pub fn diff_snapshots(before: &Snapshot, after: &Snapshot) -> Vec<DiffEntry> {
    let mut results = Vec::new();
    let after_map: HashMap<&str, &Value> = after
        .state
        .iter()
        .map(|e| (e.name.as_str(), &e.decoded_value))
        .collect();

    for entry in &before.state {
        match after_map.get(entry.name.as_str()) {
            None => results.push(DiffEntry {
                name: entry.name.clone(),
                r#type: entry.r#type.clone(),
                status: DiffStatus::Removed,
                before: Some(entry.decoded_value.clone()),
                after: None,
            }),
            Some(after_value) if **after_value != entry.decoded_value => {
                results.push(DiffEntry {
                    name: entry.name.clone(),
                    r#type: entry.r#type.clone(),
                    status: DiffStatus::Changed,
                    before: Some(entry.decoded_value.clone()),
                    after: Some((*after_value).clone()),
                })
            }
            Some(_) => results.push(DiffEntry {
                name: entry.name.clone(),
                r#type: entry.r#type.clone(),
                status: DiffStatus::Unchanged,
                before: Some(entry.decoded_value.clone()),
                after: None,
            }),
        }
    }

    let before_names: std::collections::HashSet<&str> =
        before.state.iter().map(|e| e.name.as_str()).collect();

    for entry in &after.state {
        if !before_names.contains(entry.name.as_str()) {
            results.push(DiffEntry {
                name: entry.name.clone(),
                r#type: entry.r#type.clone(),
                status: DiffStatus::Added,
                before: None,
                after: Some(entry.decoded_value.clone()),
            });
        }
    }

    results
}
```

**Milestone:** `slotprobe diff before.json after.json` produces a colored terminal output similar to `git diff`.

---

### Phase 5 — Migration Generator

**Goal:** Given a diff, generate a working Foundry or Hardhat script that migrates the changed state.

**What to build:** `src/core/migration/`

**Step 5.1 — Create the Foundry template**

```handlebars
{{! templates/foundry.hbs }}
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/{{contractName}}.sol";

contract Migrate{{contractName}} is Script {
    address constant CONTRACT = {{address}};

    function run() external {
        vm.startBroadcast();

        {{contractName}} c = {{contractName}}(CONTRACT);

        {{#each changes}}
        // Changed: {{this.name}} ({{this.before}} → {{this.after}})
        c.set{{capitalize this.name}}({{this.after}});
        {{/each}}

        vm.stopBroadcast();
    }
}
```

**Step 5.2 — Generator logic**

```rust
// src/core/migration/generator.rs
use anyhow::Result;
use handlebars::{handlebars_helper, Handlebars};
use serde_json::json;
use std::fs;

use crate::core::diff::engine::{DiffEntry, DiffStatus};

handlebars_helper!(capitalize: |s: String| {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
});

pub fn generate_migration(
    diffs: &[DiffEntry],
    contract_name: &str,
    address: &str,
    format: &str, // "foundry" | "hardhat"
) -> Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("capitalize", Box::new(capitalize));

    let template_path = format!("./templates/{format}.hbs");
    let template = fs::read_to_string(template_path)?;
    handlebars.register_template_string("migration", template)?;

    let changes: Vec<&DiffEntry> = diffs
        .iter()
        .filter(|d| matches!(d.status, DiffStatus::Changed | DiffStatus::Added))
        .collect();

    let rendered = handlebars.render(
        "migration",
        &json!({
            "contractName": contract_name,
            "address": address,
            "changes": changes,
        }),
    )?;

    Ok(rendered)
}
```

**Milestone:** `slotprobe generate-migration before.json after.json --format foundry` outputs a `.s.sol` file you can run with `forge script`.

---

### Phase 6 — Multi-Chain Consistency Checker

**Goal:** Given a contract address and list of chains, read the same variables on each chain and report divergences.

**What to build:** `src/core/cross_chain/`

**Step 6.1 — Parallel chain queries**

```rust
// src/core/cross_chain/checker.rs
use alloy::primitives::Address;
use anyhow::Result;
use futures::future::try_join_all;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::artifact_parser::types::StorageLayout;
use crate::core::snapshot::capture::{capture_snapshot, Snapshot};

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableConsistency {
    pub variable: String,
    pub consistent: bool,
    pub values: Vec<ChainValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainValue {
    pub chain: String,
    pub value: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsistencyReport {
    pub address: String,
    pub chains: Vec<String>,
    pub results: Vec<VariableConsistency>,
}

pub async fn check_consistency(
    address: Address,
    chains: &[(String, String)], // (chain_name, rpc_url)
    layout: &StorageLayout,
    variables: Option<&[String]>,
) -> Result<ConsistencyReport> {
    // Query every chain concurrently instead of sequentially.
    let snapshot_futures = chains
        .iter()
        .map(|(chain, rpc_url)| capture_snapshot(address, layout, chain, rpc_url, None));
    let snapshots: Vec<Snapshot> = try_join_all(snapshot_futures).await?;

    let filtered_vars: Vec<_> = layout
        .variables
        .iter()
        .filter(|v| variables.map_or(true, |vars| vars.iter().any(|name| name == &v.name)))
        .collect();

    let mut results = Vec::new();
    for variable in filtered_vars {
        let values: Vec<ChainValue> = snapshots
            .iter()
            .map(|s| ChainValue {
                chain: s.chain.clone(),
                value: s
                    .state
                    .iter()
                    .find(|e| e.name == variable.name)
                    .map(|e| e.decoded_value.clone()),
            })
            .collect();

        let unique: std::collections::HashSet<String> = values
            .iter()
            .map(|v| serde_json::to_string(&v.value).unwrap_or_default())
            .collect();

        results.push(VariableConsistency {
            variable: variable.name.clone(),
            consistent: unique.len() == 1,
            values,
        });
    }

    Ok(ConsistencyReport {
        address: format!("{address:#x}"),
        chains: chains.iter().map(|(name, _)| name.clone()).collect(),
        results,
    })
}
```

**Milestone:** `slotprobe cross-chain 0x... --chains mainnet,arbitrum,base --vars fee,tickSpacing` outputs a clear table showing which variables match across chains and which have drifted.

---

### Phase 7 — CLI Interface

**Goal:** Wire all the above into a clean, polished CLI with `clap`'s derive API.

**What to build:** `src/cli/`

```rust
// src/main.rs
mod cli;
mod core;
mod rpc;
mod config;

use clap::{Parser, Subcommand};
use anyhow::Result;

use cli::commands::{check_collision, cross_chain, diff, generate_migration, snapshot};

#[derive(Parser)]
#[command(
    name = "slotprobe",
    version = "0.1.0",
    about = "Smart contract state diffing, collision detection, and migration tooling"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Snapshot(snapshot::SnapshotArgs),
    Diff(diff::DiffArgs),
    CheckCollision(check_collision::CheckCollisionArgs),
    CrossChain(cross_chain::CrossChainArgs),
    GenerateMigration(generate_migration::GenerateMigrationArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Snapshot(args) => snapshot::run(args).await?,
        Commands::Diff(args) => diff::run(args).await?,
        Commands::CheckCollision(args) => check_collision::run(args).await?,
        Commands::CrossChain(args) => cross_chain::run(args).await?,
        Commands::GenerateMigration(args) => generate_migration::run(args).await?,
    }

    Ok(())
}
```

Each subcommand module defines its own `#[derive(clap::Args)]` struct and an `async fn run(args) -> Result<()>`, so adding a new command is just: new file, new `Args` struct, one new match arm.

**Milestone:** `cargo build --release` and then run `./target/release/slotprobe --help` and see all subcommands listed cleanly. `cargo install --path .` makes `slotprobe` available globally.

---

### Phase 8 — Foundry & Hardhat Integration

**Goal:** Let devs run SlotProbe from within their existing Foundry or Hardhat workflow without switching to a separate CLI.

**Hardhat Integration:**

Since Hardhat plugins are TypeScript/JS-native, the cleanest bridge from a Rust binary is a thin JS wrapper task that shells out to the compiled `slotprobe` binary and parses its JSON output:

```typescript
// A small Hardhat task that just invokes the Rust binary
import { task } from "hardhat/config";
import { execFile } from "node:child_process";
import { promisify } from "node:util";

const run = promisify(execFile);

task("slotprobe:snapshot", "Take a storage state snapshot").setAction(async (args) => {
  const { stdout } = await run("slotprobe", ["snapshot", ...toCliArgs(args)]);
  console.log(stdout);
});
```

**Foundry Integration:**

Foundry doesn't have a plugin system, and since both Foundry and SlotProbe are native binaries, integration is even more natural here — call `slotprobe` directly as a pre- or post-step in your `forge script` workflow, or from a `Makefile`/CI job:

```bash
forge build
slotprobe check-collision ./old/MyContract.json ./out/MyContract.sol/MyContract.json
forge script script/Upgrade.s.sol --broadcast
slotprobe snapshot $CONTRACT_ADDRESS --chain mainnet --out after.json
```

---

## 🌟 Standout Features to Add

These will separate SlotProbe from everything else and make it go viral:

### 1. AI-Powered Migration Suggestions

Integrate the Anthropic API (via a simple `reqwest` HTTP client, since there's no official Rust SDK). When a diff is detected, send the variable names and types to Claude and ask it to suggest what the migration should do semantically — not just mechanically copy values, but reason about *why* they changed and whether the migration script is logically correct.

### 2. "Run Against the Top 50 Protocols" Mode

Build a script that runs SlotProbe against the top 50 DeFi protocols by TVL, checking for storage layout issues in their latest upgrades. This is your launch content. Publish the results.

### 3. GitHub Actions Integration

Provide a ready-made GitHub Actions workflow file that downloads a prebuilt static binary (no compile step needed for CI users) and runs it directly:

```yaml
- name: Check storage collisions
  uses: slotprobe/action@v1
  with:
    old-artifact: './artifacts/old/MyContract.json'
    new-artifact: './artifacts/new/MyContract.json'
    fail-on-collision: true
```

### 4. VS Code Extension

A sidebar panel that shows the storage layout of the currently open Solidity file, with live values pulled from a configured network. The extension itself stays TypeScript (VS Code extensions require the Node.js extension host), but it shells out to the Rust `slotprobe` binary for all the actual logic, communicating over JSON on stdout.

### 5. Snapshot Timeline

Store multiple snapshots over time and generate a timeline view showing how each variable's value changed block by block. Useful for post-incident analysis.

### 6. Etherscan Integration

For contracts without local artifact files (e.g. deployed by someone else), automatically fetch verified source from Etherscan via `reqwest` and compile it on-the-fly (shelling out to `solc` or using the `svm-rs`/`foundry-compilers` crates) to get the storage layout.

### 7. Simulation Mode

Before running an actual migration, simulate it against an Anvil fork and verify all state was correctly migrated, then produce a verification report.

---

## 🧪 Testing Strategy

### Unit Tests (`cargo test`)

Test each pure function in isolation:
- Slot calculator: verify known keccak256 outputs for mapping and array slots
- Decoder: verify each Solidity type decodes correctly from known hex values
- Artifact parser: test against fixture JSON files from both Foundry and Hardhat
- Diff engine: test all four diff statuses (added, removed, changed, unchanged)

### Integration Tests (`cargo test` + Anvil)

Fork mainnet and test against real contracts with known storage:

```rust
// tests/integration/snapshot.rs
use slotprobe::core::snapshot::capture::capture_snapshot;

// Start an Anvil fork before tests (e.g. via the `alloy-node-bindings`
// crate's Anvil helper, or spawn it manually with std::process::Command),
// point your RPC client at it.
// Use a well-known contract like USDC or Uniswap V3 WETH/USDC pool.
// Assert that key variables (e.g. total_supply, decimals) decode correctly.

#[tokio::test]
async fn correctly_reads_usdc_total_supply_from_mainnet_fork() {
    // ...
}
```

### End-to-End Tests

A `tests/` binary-driving test (or a `xtask`-style helper) that runs the full CLI flow — snapshot → diff → generate-migration — against a locally deployed upgradeable contract on Anvil, using `assert_cmd` to invoke the compiled binary and verify the output files.

---

## 📍 Roadmap

**v0.1 — Core (Build this first)**
- [x] EVM storage engine (slot reading + decoding)
- [x] Artifact parser (Foundry + Hardhat)
- [x] Snapshot capture and JSON serialization
- [x] Semantic diff engine
- [x] Terminal output with color

**v0.2 — Power Features**
- [ ] Storage collision detector
- [ ] Migration script generator (Foundry)
- [ ] Cross-chain consistency checker

**v0.3 — Ecosystem Integration**
- [ ] Hardhat task wrapper (thin JS shim over the Rust binary)
- [ ] GitHub Actions workflow with prebuilt static binaries
- [ ] Etherscan auto-fetch for unverified artifacts

**v0.4 — Standout**
- [ ] AI migration suggestions
- [ ] VS Code extension (shells out to the Rust binary)
- [ ] Snapshot timeline viewer
- [ ] HTML report output

---

## 🤝 Contributing

SlotProbe is open source under the MIT License. Contributions are welcome and encouraged.

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/your-feature`)
3. Write tests for what you build (`cargo test`)
4. Run `cargo fmt` and `cargo clippy --all-targets -- -D warnings` before submitting
5. Submit a pull request with a clear description

If you find a storage bug in a real protocol using SlotProbe, please disclose responsibly to the protocol team first, then share your findings publicly. The Web3 community gets stronger when we work together.

---

## 📄 License

MIT © SlotProbe Contributors

---

*Built by developers who got tired of hunting storage slots manually.*