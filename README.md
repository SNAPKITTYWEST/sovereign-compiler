# root-fontana

> Constitutional compiler for the SnapKitty ecosystem.
> Witness. Archivum. Governance. Contractivity. Observatory. Execution.

[![License: Sovereign Source](https://img.shields.io/badge/License-Sovereign%20Source-blue.svg)](SOVEREIGN.md)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Lean](https://img.shields.io/badge/Lean-4-blue.svg)](https://lean-lang.org/)

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    ROOT FONTANA CONSTITUTIONAL COMPILER          │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │                    Source Artifact                         │  │
│   │              (JSON / Fontana DSL)                         │  │
│   └──────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │                 Fontana DSL Parser                        │  │
│   │         Grammar → AST → Admissibility Check              │  │
│   └──────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │                  Unified Witness                          │  │
│   │            (artifact + label + receipt)                   │  │
│   └──────────────────────────────────────────────────────────┘  │
│                           │                                      │
│          ┌────────────────┼────────────────┐                    │
│          ▼                ▼                ▼                    │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────────┐       │
│  │   Archivum   │ │  Governance  │ │  Contractivity   │       │
│  │   (ledger)   │ │  (engine)    │ │  (receipts)      │       │
│  └──────────────┘ └──────────────┘ └──────────────────┘       │
│          │                │                │                    │
│          └────────────────┼────────────────┘                    │
│                           ▼                                      │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │                  Observatory                              │  │
│   │            (telemetry + metrics)                          │  │
│   └──────────────────────────────────────────────────────────┘  │
│                           │                                      │
│                           ▼                                      │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │                  Execution Engine                          │  │
│   │            (deterministic runtime)                        │  │
│   └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│   ┌──────────────────────────────────────────────────────────┐  │
│   │                  Lean 4 Proofs                             │  │
│   │         Contractivity.lean  Strata.lean                   │  │
│   │         Verification.lean   RootFontana.lean              │  │
│   └──────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Module Overview

| Module | Language | Description |
|--------|----------|-------------|
| **witness** | Rust | UnifiedWitness, WORM Witness types |
| **archivum** | Rust | Immutable ledger, append-only |
| **governance** | Rust | Governance engine, approval workflows |
| **contractivity** | Rust | SHA-256 cryptographic receipts |
| **observatory** | Rust | Telemetry, metrics, dashboards |
| **execution** | Rust | Deterministic runtime engine |
| **RootFontana.lean** | Lean 4 | Core constitutional axioms |
| **Contractivity.lean** | Lean 4 | Contractivity proof |
| **Strata.lean** | Lean 4 | Stratum boundary proof |
| **Verification.lean** | Lean 4 | Verification proof |
| **fontana-dsl** | Rust | Grammar, AST, Admissibility |

## Fontana DSL Grammar

```bnf
<program>     ::= <declaration>*
<declaration> ::= <artifact-decl> | <governance-decl> | <execution-decl>
<artifact-decl> ::= "artifact" <identifier> "{" <field>* "}"
<governance-decl> ::= "governance" <identifier> "{" <rule>* "}"
<execution-decl> ::= "execute" <identifier> "{" <step>* "}"
<field>       ::= <identifier> ":" <type> ";"
<rule>        ::= "allow" <condition> "->" <action> ";"
<step>        ::= <identifier> ":=" <expression> ";"
<expression>  ::= <literal> | <identifier> | <binary-op>
<binary-op>   ::= <expression> <operator> <expression>
<operator>    ::= "+" | "-" | "*" | "/" | "=="
<type>        ::= "int" | "string" | "bool" | "hash"
<literal>     ::= <integer> | <string> | <boolean> | <hash>
```

## Quick Start

### Build

```bash
# Rust components
cargo build --release

# Lean 4 proofs
lean --version  # Ensure Lean 4 is installed
lake build
```

### CLI Usage

```bash
# Compile artifact with governance
fontana compile artifact.json --governance governance.fontana

# Generate WORM witness
fontana witness artifact.json
# Output: {"artifact_hash":"...","label":"...","status":"accepted",...}

# Verify contractivity receipt
fontana verify receipt.json

# Run execution engine
fontana execute program.fontana

# View archivum ledger
fontana ledger --last 10
```

### Rust API

```rust
use root_fontana::{
    UnifiedWitness, WORMWitness,
    Archivum, GovernanceEngine,
    ContractivityReceipt, Observatory,
};

fn main() {
    // Create witness
    let witness = UnifiedWitness::new(artifact, label);
    
    // Seal as WORM
    let worm = WORMWitness::seal(witness);
    
    // Add to ledger
    let mut archivum = Archivum::new();
    archivum.append(worm.clone());
    
    // Check governance
    let governance = GovernanceEngine::new();
    let approved = governance.check(&worm);
    
    // Generate receipt
    let receipt = ContractivityReceipt::new(input, output);
    let seal = receipt.seal();
    
    // Record telemetry
    let mut observatory = Observatory::new();
    observatory.record("artifact.processed", 1);
}
```

## Interactive Demo

```bash
# Demo 1: Compile artifact
$ fontana compile artifact.json --governance governance.fontana
Compiling artifact...
  ✓ Artifact validated
  ✓ Governance rules applied
  ✓ WORM witness generated
  ✓ Receipt sealed
Output: output/fontana_witness.json

# Demo 2: Verify receipt
$ fontana verify receipt.json
{
  "status": "valid",
  "artifact_hash": "sha256:a1b2c3d4...",
  "receipt_hash": "sha256:e5f6a7b8...",
  "seal": "sha256:c9d0e1f2..."
}

# Demo 3: View ledger
$ fontana ledger --last 5
┌─────────────────────────────────────────────────────────────┐
│                    Archivum Ledger                           │
├─────────────────────────────────────────────────────────────┤
│ #  │ Artifact Hash      │ Status    │ Timestamp             │
├─────────────────────────────────────────────────────────────┤
│ 1  │ sha256:a1b2c3d4... │ accepted  │ 2025-01-01T00:00:00Z  │
│ 2  │ sha256:e5f6a7b8... │ accepted  │ 2025-01-01T00:01:00Z  │
│ 3  │ sha256:c9d0e1f2... │ rejected  │ 2025-01-01T00:02:00Z  │
│ 4  │ sha256:d3b4c5a6... │ accepted  │ 2025-01-01T00:03:00Z  │
│ 5  │ sha256:f7e8d9c0... │ accepted  │ 2025-01-01T00:04:00Z  │
└─────────────────────────────────────────────────────────────┘
```

## Lean 4 Proofs

| Proof | File | Statement |
|-------|------|-----------|
| **Contractivity** | `Contractivity.lean` | ∀ f, contractive(f) → ∃! x, f(x) = x |
| **Strata** | `Strata.lean` | ∀ n, stratum(n) ∧ ¬stratum(n+1) |
| **Verification** | `Verification.lean` | ∀ w, verified(w) → authentic(w) |
| **Root** | `RootFontana.lean` | ∀ a, admissible(a) → accepted(a) |

## Invariants

| Invariant | Description |
|-----------|-------------|
| **No Recursion** | All computation is staged |
| **Constitutional** | All rules are declared in DSL |
| **WORM Sealed** | Every witness is write-once |
| **Mathematically Verified** | Lean 4 proofs |
| **Observable** | Every state change is logged |

## Testing

```bash
# Rust tests
cargo test

# Lean 4 proofs
lake build

# All tests
cargo test && lake build
```

## License

Sovereign Source License — see [SOVEREIGN.md](SOVEREIGN.md)

---

```
ROOT-FONTANA-001
Witness. Archivum. Govern. Contract. Observe. Execute.
Same artifact. Same witness.
No recursion. No borrowed thesis.
```
