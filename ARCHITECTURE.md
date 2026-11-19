# Architecture

Keep this file short (~1 page). Project-specific invariants live here; universal Rust workflow lives in `AGENTS.md`.

## Purpose
<!-- One paragraph: what this crate does and what it explicitly does NOT do. -->

## Module Boundaries
<!-- Top-level modules and their allowed dependency direction. -->
- `src/...`: ...

## Core Invariants
<!-- 3-7 non-negotiable rules (e.g., sans-I/O core, zero-allocation hot path, unsafe quarantine). -->
- ...

## Forbidden Patterns / Dependencies
<!-- Anything agents must never introduce in specific modules. -->
- ...
