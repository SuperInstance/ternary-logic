# Future Integration: ternary-logic

## Current State
Supports Kleene (strong), Łukasiewicz (weak), Bochvar internal, and Gödel-Dummett three-valued logics with truth tables, entailment relations, tautology checking, and modal extensions.

## Integration Opportunities

### With ternary-cell (Reasoning About State)
Cell state is ternary, but cell reasoning needs logic. When a cell observes {-1, 0, +1} inputs and must decide its next state, it needs a logic system. Kleene strong three-valued logic handles the "unknown" case gracefully — if inputs are unknown, the output propagates unknown rather than defaulting to false. This is safer for room state management than binary logic.

### With ternary-protocol (Message Validation)
Protocol messages carry ternary payloads. Validation needs logic: is this message entailed by previous messages? `LogicSystem::entails()` checks. Different logic systems for different security levels: Kleene for general use, Bochvar for internal-only messages (internal logic rejects any unknown input), Łukasiewicz for probabilistic reasoning.

### With ternary-circuit
Circuit design uses logic gates. `ternary-logic` defines the truth tables that `ternary-circuit` implements. Each `LogicSystem` corresponds to a different gate design — Kleene gates vs Łukasiewicz gates. Together they form the theoretical + physical layers of ternary computing.

## Potential in Mature Systems
In room-as-codespace, three-valued logic is the reasoning framework. A room doesn't just have "active/inactive" — it has "active/unknown/inactive." Unknown rooms might exist (Codespace starting up) or might be unreachable (network partition). Łukasiewicz logic handles probabilistic room state; Kleene handles strict unknown propagation.

## Cross-Pollination Ideas
- Modal logic extensions for reasoning about room capabilities: "possibly has skill X" vs "necessarily has skill X"
- Tautology checking for invariant verification — prove that conservation laws hold under ternary logic
- Entailment as the basis for room permission inference: if room A entails capability X, and X entails access to room B, then A → B

## Dependencies for Next Steps
- ternary-cell needs a logic system parameter for state update rules
- ternary-protocol needs entailment checking for message chains
- Integration with ternary-circuit for logic-to-gate mapping
