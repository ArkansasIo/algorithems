# Algorithm Engine

A Rust workspace for a composable mathematical algorithm engine. Algorithms are represented as objects, statements, graphs and trees and can be composed, validated, transformed and generated.

## Architecture

- `crates/algorithm-core` — values, contexts, statements, nodes and specifications
- `crates/algorithm-math` — number theory and foundational mathematics
- `crates/algorithm-symbolic` — symbolic expression model
- `crates/algorithm-generator` — algorithm synthesis pipeline
- `crates/algorithm-language` — initial DSL/tokenizer foundation
- `crates/algorithm-api` — public API facade
- `crates/algorithm-cli` — command-line demonstration

## Generator pipeline

`Problem -> Specification -> Graph -> Candidate Algorithms -> Validation -> Optimization -> Generated Algorithm`

This project is an independent mathematical computation framework; it does not reproduce proprietary Wolfram Alpha implementation or data.

## Build

```bash
cargo check
cargo test
cargo run -p algorithm-cli
```
