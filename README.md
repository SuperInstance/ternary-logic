# ternary-logic

Advanced ternary logic systems — Kleene (K3), Łukasiewicz (L3), Bochvar (B3), and Gödel-Dummett (G3) three-valued logics with truth tables, entailment, tautology checking, and modal operators.

## Why This Exists

Classical logic assumes every proposition is either true or false. Reality is messier: database queries return "unknown," programs hit "null" values, and AI systems face genuine uncertainty. Three-valued logic handles this by introducing a third truth value — **Unknown** — alongside True and False. But "unknown" means different things in different contexts, and each interpretation gives rise to a different logic system.

This crate implements four major three-valued logic systems as a unified library:
- **Kleene (K3)**: Unknown propagates conservatively — True ∧ Unknown = Unknown, False ∧ Unknown = False.
- **Łukasiewicz (L3)**: Adds a paracomplete implication where Unknown → Unknown is True.
- **Bochvar (B3)**: Unknown is toxic — any operation touching Unknown returns Unknown.
- **Gödel-Dummett (G3)**: Uses min/max ordering with a unique implication where Unknown → Unknown = True.

Each system defines its own truth tables for conjunction, disjunction, implication, and equivalence, plus modal operators (necessity □ and possibility ◇). The crate supports formula evaluation, tautology checking, entailment relations, and truth table generation — making it a complete toolkit for ternary logic research and application.

This crate is part of the **Negative Space Intelligence** ecosystem.

## Core Concepts

- **Ternary** — A truth value: `False` (0), `Unknown` (1), or `True` (2).
- **LogicSystem** — Which logic to use: `Kleene`, `Lukasiewicz`, `Bochvar`, or `GödelDummett`.
- **UnaryOp / BinaryOp** — Operations: Negation, Necessity, Possibility, Conjunction, Disjunction, Implication, Equivalence.
- **Formula** — A recursive propositional formula with atoms, connectives, and modal operators. Evaluates under any logic system.
- **Truth Tables** — Generate 3×3 tables for binary ops or 3-element tables for unary ops.
- **Entailment** — Check if A entails B (A → B is True).
- **Tautology Checking** — Verify a formula is True for all possible inputs.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-logic = "0.1"
```

```rust
use ternary_logic::*;

// Evaluate operations under different logic systems
let systems = [LogicSystem::Kleene, LogicSystem::Lukasiewicz,
               LogicSystem::Bochvar, LogicSystem::GödelDummett];

// Negation: True → False, False → True, Unknown → Unknown
assert_eq!(negate(Ternary::True, LogicSystem::Kleene), Ternary::False);
assert_eq!(negate(Ternary::Unknown, LogicSystem::Kleene), Ternary::Unknown);

// Kleene: Unknown propagates conservatively
assert_eq!(kleene_conj(Ternary::True, Ternary::Unknown), Ternary::Unknown);
assert_eq!(kleene_conj(Ternary::False, Ternary::Unknown), Ternary::False);

// Łukasiewicz: Unknown → Unknown is True!
assert_eq!(lukasiewicz_impl(Ternary::Unknown, Ternary::Unknown), Ternary::True);

// Bochvar: Unknown poisons everything
assert_eq!(bochvar_conj(Ternary::True, Ternary::Unknown), Ternary::Unknown);

// Build and evaluate formulas
let formula = Formula::And(
    Box::new(Formula::Atom(Ternary::True)),
    Box::new(Formula::Not(Box::new(Formula::Atom(Ternary::Unknown)))),
);
assert_eq!(formula.eval(LogicSystem::Kleene), Ternary::Unknown);

// Modal operators
assert_eq!(necessity(Ternary::True, LogicSystem::Kleene), Ternary::True);
assert_eq!(necessity(Ternary::Unknown, LogicSystem::Kleene), Ternary::False);
assert_eq!(possibility(Ternary::Unknown, LogicSystem::Kleene), Ternary::True);

// Check tautologies
let is_identity = is_tautology_binary(|a, _b, sys| {
    binary_op(BinaryOp::Implication, a, a, sys)
}, LogicSystem::Lukasiewicz);
assert!(is_identity); // a → a is a tautology in L3

// Excluded middle is NOT a tautology in L3
let is_lem = is_tautology_binary(|a, _b, sys| {
    binary_op(BinaryOp::Disjunction, a, negate(a, sys), sys)
}, LogicSystem::Lukasiewicz);
assert!(!is_lem);

// Generate and analyze truth tables
let table = truth_table_binary(BinaryOp::Conjunction, LogicSystem::Kleene);
let designated = count_designated_binary(&table); // count True entries
let unknowns = count_unknown_binary(&table);
```

## API Overview

### Core Types
| Type | Description |
|---|---|
| `Ternary` | Truth value: `False`, `Unknown`, `True` |
| `LogicSystem` | `Kleene`, `Lukasiewicz`, `Bochvar`, `GödelDummett` |
| `UnaryOp` | `Negation`, `Necessity`, `Possibility` |
| `BinaryOp` | `Conjunction`, `Disjunction`, `Implication`, `Equivalence` |
| `Formula` | Recursive formula tree with atoms and connectives |

### Operations
| Function | Description |
|---|---|
| `unary_op(op, val, system)` | Evaluate unary operation |
| `binary_op(op, a, b, system)` | Evaluate binary operation |
| `necessity(val, system)` | Modal □ (only True → True) |
| `possibility(val, system)` | Modal ◇ (not False → True) |
| `entails(a, b, system)` | Does A entail B? |
| `truth_table_binary(op, system)` | Generate 3×3 truth table |
| `truth_table_unary(op, system)` | Generate 3-element table |
| `is_tautology_unary(f, system)` | Always True? |
| `is_tautology_binary(f, system)` | Always True? |

### Formula Variants
| Variant | Description |
|---|---|
| `Atom(Ternary)` | Literal value |
| `Not(inner)` | Negation |
| `And(l, r)` / `Or(l, r)` | Conjunction / Disjunction |
| `Implies(l, r)` / `Equiv(l, r)` | Implication / Equivalence |
| `Necessity(inner)` / `Possibility(inner)` | Modal operators |

## How It Works

Each logic system defines its own semantics for the four binary connectives, but all share the same negation (True↔False, Unknown stays). The key differences lie in **implication** and how Unknown is handled:

**Kleene (K3)** treats Unknown as "genuinely could be either." Conjunction with Unknown is Unknown unless one operand is False (which forces False). Implication is defined as ¬A ∨ B. K3 has no tautologies because any formula containing Unknown evaluates to Unknown.

**Łukasiewicz (L3)** defines a special implication where Unknown → Unknown = True, making it paracomplete — it can tolerate contradictions without explosion. This makes identity (a → a) a tautology in L3, even though excluded middle (a ∨ ¬a) is not.

**Bochvar (B3)** treats Unknown as a "meaningless" or "undefined" value that contaminates everything. Any binary operation involving Unknown returns Unknown, making it useful for error propagation and null-like semantics.

**Gödel-Dummett (G3)** uses the ordering False < Unknown < True. Conjunction takes the minimum, disjunction takes the maximum. Its implication makes Unknown → Unknown = True, similar to L3, but with different truth tables overall due to the ordering semantics.

## Use Cases

1. **Database query processing** — SQL uses an Unknown truth value for NULL comparisons. Kleene logic matches SQL's three-valued semantics for WHERE clause evaluation.

2. **Formal verification** — Model checking with ternary states (satisfied/violated/unknown). Bochvar logic ensures that unverified portions propagate as Unknown rather than being silently accepted.

3. **AI reasoning under uncertainty** — When an AI can't determine True or False, Łukasiewicz logic provides a coherent framework for reasoning that avoids both explosion (from contradictions) and false precision.

4. **Programming language semantics** — Model short-circuit evaluation, nullable types, or option types. The four logics capture different design choices for handling "missing" values.

## Ecosystem

| Crate | Relationship |
|---|---|
| `ternary-hardware` | Trit operations use Kleene-style AND/OR |
| `ternary-bayesian` | Probabilistic extension of deterministic logic |
| `ternary-quantum` | Qutrit measurement collapses ternary logic to definite values |
| `ternary-attention` | Attention compatibility uses ternary conjunction |
| `ternary-locks` | Lock composition uses AND/OR/NOT from ternary logic |

## License

MIT
