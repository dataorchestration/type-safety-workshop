# Make Invalid Data Unrepresentable

**Rust's Type System for Data Engineering — A Hands-On Workshop (90 min)**

> Walk in thinking type safety is about `int vs string`.
> Walk out knowing the compiler can enforce your entire data pipeline's correctness — for free.

## Quick Start

```bash
# Clone
git clone https://github.com/dataorc/type-safety-workshop.git
cd type-safety-workshop

# Setup (installs Rust if needed, verifies everything)
# macOS / Linux / WSL:
chmod +x scripts/setup.sh && ./scripts/setup.sh

# Windows (PowerShell):
powershell -ExecutionPolicy Bypass -File scripts\setup.ps1

# Verify it works
cargo run --bin exercise_01_newtypes
```

## What You'll Build

A type-safe analytics pipeline where **if it compiles, the data is structurally correct.**

You'll learn four patterns that production engines like DataFusion and Polars use:

| # | Exercise | Pattern | What it prevents |
|---|----------|---------|-----------------|
| 1 | `exercise_01_newtypes` | Newtype | Mixing up CustomerId and OrderId — silent bug in Python, compile error in Rust |
| 2 | `exercise_02_phantom` | Phantom Types | Querying unvalidated data — runtime crash in Spark, compile error in Rust |
| 3 | `exercise_03_transforms` | Trait Bounds | Calling sum() on a string column — silent bug in pandas, compile error in Rust |
| 4 | `exercise_04_typestate` | Typestate | Executing an incomplete SQL query — runtime error in Java, compile error in Rust |

## Running Exercises

```bash
# Work on exercises (starter code with TODOs)
cargo run --bin exercise_01_newtypes
cargo run --bin exercise_02_phantom
cargo run --bin exercise_03_transforms
cargo run --bin exercise_04_typestate

# Check your work
cargo test --bin exercise_01_newtypes
cargo test --bin exercise_02_phantom
cargo test --bin exercise_03_transforms
cargo test --bin exercise_04_typestate

# See the full pipeline (all 4 patterns combined)
cargo run --bin full_pipeline

# Peek at solutions (only if stuck!)
cargo run --bin solution_01_newtypes
```

## Prerequisites

- Basic Rust: ownership, structs, enums, traits, generics
- Familiarity with data concepts: tables, columns, schemas
- Laptop with Rust toolchain (the setup script handles this)

## Repo Structure

```
type-safety-workshop/
├── Cargo.toml
├── scripts/
│   ├── setup.sh          # macOS / Linux / WSL setup
│   └── setup.ps1         # Windows PowerShell setup
├── skill/
│   └── SKILL.md          # Claude Code skill for type-safe pipelines
└── src/
    ├── exercises/         # Starter code (work on these)
    │   ├── ex01_newtypes.rs
    │   ├── ex02_phantom.rs
    │   ├── ex03_transforms.rs
    │   └── ex04_typestate.rs
    ├── solutions/         # Reference solutions
    │   ├── sol01_newtypes.rs
    │   ├── sol02_phantom.rs
    │   ├── sol03_transforms.rs
    │   └── sol04_typestate.rs
    └── full_pipeline.rs   # All 4 patterns in one pipeline
```

## Workshop Facilitator

**Navdeep** — Founder of [Dataorc](https://dataorc.io) and [OrcaSheets](https://orcasheets.io). Built OrcaSheets using DataFusion, Polars, Arrow, and Tauri — where these type-safety patterns run in production every day.

## License

MIT
