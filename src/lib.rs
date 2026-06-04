#![forbid(unsafe_code)]

//! Advanced ternary logic systems.
//!
//! Supports Kleene (strong), Łukasiewicz (weak), Bochvar internal,
//! and Gödel-Dummett three-valued logics with truth tables, entailment
//! relations, tautology checking, and modal extensions.

/// Ternary truth value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Ternary {
    /// Definitely false.
    False,
    /// Unknown / indeterminate.
    Unknown,
    /// Definitely true.
    True,
}

impl Ternary {
    /// Numeric encoding: False=0, Unknown=1, True=2.
    pub fn to_u8(self) -> u8 {
        match self {
            Ternary::False => 0,
            Ternary::Unknown => 1,
            Ternary::True => 2,
        }
    }

    /// Decode from numeric encoding.
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Ternary::False),
            1 => Some(Ternary::Unknown),
            2 => Some(Ternary::True),
            _ => None,
        }
    }

    /// All three values in order.
    pub fn all() -> &'static [Ternary; 3] {
        &[Ternary::False, Ternary::Unknown, Ternary::True]
    }

    /// Is this a classical (non-unknown) value?
    pub fn is_classical(self) -> bool {
        self != Ternary::Unknown
    }
}

impl std::fmt::Display for Ternary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ternary::False => write!(f, "F"),
            Ternary::Unknown => write!(f, "U"),
            Ternary::True => write!(f, "T"),
        }
    }
}

/// Which logic system to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogicSystem {
    /// Kleene strong three-valued logic (K3).
    Kleene,
    /// Łukasiewicz weak three-valued logic (L3).
    Lukasiewicz,
    /// Bochvar internal logic (B3).
    Bochvar,
    /// Gödel-Dummett logic (G3).
    GödelDummett,
}

/// Evaluate a unary operation under the given logic.
pub fn unary_op(op: UnaryOp, val: Ternary, system: LogicSystem) -> Ternary {
    match op {
        UnaryOp::Negation => negate(val, system),
        UnaryOp::Necessity => necessity(val, system),
        UnaryOp::Possibility => possibility(val, system),
    }
}

/// Unary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Negation,
    Necessity,
    Possibility,
}

/// Binary operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Conjunction,
    Disjunction,
    Implication,
    Equivalence,
}

fn negate(val: Ternary, _system: LogicSystem) -> Ternary {
    match val {
        Ternary::True => Ternary::False,
        Ternary::False => Ternary::True,
        Ternary::Unknown => Ternary::Unknown,
    }
}

fn kleene_conj(a: Ternary, b: Ternary) -> Ternary {
    match (a, b) {
        (Ternary::False, _) | (_, Ternary::False) => Ternary::False,
        (Ternary::Unknown, _) | (_, Ternary::Unknown) => Ternary::Unknown,
        _ => Ternary::True,
    }
}

fn kleene_disj(a: Ternary, b: Ternary) -> Ternary {
    match (a, b) {
        (Ternary::True, _) | (_, Ternary::True) => Ternary::True,
        (Ternary::Unknown, _) | (_, Ternary::Unknown) => Ternary::Unknown,
        _ => Ternary::False,
    }
}

fn kleene_impl(a: Ternary, b: Ternary) -> Ternary {
    kleene_disj(negate(a, LogicSystem::Kleene), b)
}

fn lukasiewicz_impl(a: Ternary, b: Ternary) -> Ternary {
    match (a, b) {
        (Ternary::False, _) => Ternary::True,
        (Ternary::True, Ternary::True) => Ternary::True,
        (Ternary::Unknown, Ternary::True) => Ternary::True,
        (Ternary::True, Ternary::False) => Ternary::False,
        (Ternary::Unknown, Ternary::Unknown) => Ternary::True,
        (Ternary::True, Ternary::Unknown) => Ternary::Unknown,
        (Ternary::Unknown, Ternary::False) => Ternary::Unknown,
        _ => Ternary::Unknown,
    }
}

fn bochvar_conj(a: Ternary, b: Ternary) -> Ternary {
    if a == Ternary::Unknown || b == Ternary::Unknown {
        Ternary::Unknown
    } else {
        kleene_conj(a, b)
    }
}

fn bochvar_disj(a: Ternary, b: Ternary) -> Ternary {
    if a == Ternary::Unknown || b == Ternary::Unknown {
        Ternary::Unknown
    } else {
        kleene_disj(a, b)
    }
}

fn godel_impl(a: Ternary, b: Ternary) -> Ternary {
    match (a, b) {
        (Ternary::False, _) => Ternary::True,
        (_, Ternary::True) => Ternary::True,
        (Ternary::True, Ternary::False) => Ternary::False,
        (Ternary::True, Ternary::Unknown) => Ternary::Unknown,
        (Ternary::Unknown, Ternary::False) => Ternary::False,
        (Ternary::Unknown, Ternary::Unknown) => Ternary::True,
    }
}

fn godel_conj(a: Ternary, b: Ternary) -> Ternary {
    use std::cmp::Ordering;
    match a.to_u8().cmp(&b.to_u8()) {
        Ordering::Less => a,
        Ordering::Greater => b,
        Ordering::Equal => a,
    }
}

/// Evaluate a binary operation under the given logic.
pub fn binary_op(op: BinaryOp, a: Ternary, b: Ternary, system: LogicSystem) -> Ternary {
    match system {
        LogicSystem::Kleene => match op {
            BinaryOp::Conjunction => kleene_conj(a, b),
            BinaryOp::Disjunction => kleene_disj(a, b),
            BinaryOp::Implication => kleene_impl(a, b),
            BinaryOp::Equivalence => {
                kleene_conj(kleene_impl(a, b), kleene_impl(b, a))
            }
        },
        LogicSystem::Lukasiewicz => match op {
            BinaryOp::Conjunction => kleene_conj(a, b),
            BinaryOp::Disjunction => kleene_disj(a, b),
            BinaryOp::Implication => lukasiewicz_impl(a, b),
            BinaryOp::Equivalence => {
                kleene_conj(lukasiewicz_impl(a, b), lukasiewicz_impl(b, a))
            }
        },
        LogicSystem::Bochvar => match op {
            BinaryOp::Conjunction => bochvar_conj(a, b),
            BinaryOp::Disjunction => bochvar_disj(a, b),
            BinaryOp::Implication => {
                if a == Ternary::Unknown || b == Ternary::Unknown {
                    Ternary::Unknown
                } else {
                    kleene_impl(a, b)
                }
            }
            BinaryOp::Equivalence => {
                if a == Ternary::Unknown || b == Ternary::Unknown {
                    Ternary::Unknown
                } else {
                    kleene_conj(kleene_impl(a, b), kleene_impl(b, a))
                }
            }
        },
        LogicSystem::GödelDummett => match op {
            BinaryOp::Conjunction => godel_conj(a, b),
            BinaryOp::Disjunction => kleene_disj(a, b),
            BinaryOp::Implication => godel_impl(a, b),
            BinaryOp::Equivalence => {
                godel_conj(godel_impl(a, b), godel_impl(b, a))
            }
        },
    }
}

/// Necessity modal operator (box).
pub fn necessity(val: Ternary, system: LogicSystem) -> Ternary {
    match system {
        LogicSystem::Kleene | LogicSystem::Lukasiewicz => {
            match val {
                Ternary::True => Ternary::True,
                _ => Ternary::False,
            }
        }
        LogicSystem::Bochvar => {
            match val {
                Ternary::True => Ternary::True,
                Ternary::Unknown => Ternary::Unknown,
                Ternary::False => Ternary::False,
            }
        }
        LogicSystem::GödelDummett => {
            match val {
                Ternary::True => Ternary::True,
                _ => Ternary::False,
            }
        }
    }
}

/// Possibility modal operator (diamond).
pub fn possibility(val: Ternary, system: LogicSystem) -> Ternary {
    negate(necessity(negate(val, system), system), system)
}

/// A truth table for a binary operation under a given logic.
pub fn truth_table_binary(op: BinaryOp, system: LogicSystem) -> [[Ternary; 3]; 3] {
    let mut table = [[Ternary::False; 3]; 3];
    for (i, &a) in Ternary::all().iter().enumerate() {
        for (j, &b) in Ternary::all().iter().enumerate() {
            table[i][j] = binary_op(op, a, b, system);
        }
    }
    table
}

/// A truth table for a unary operation.
pub fn truth_table_unary(op: UnaryOp, system: LogicSystem) -> [Ternary; 3] {
    let mut table = [Ternary::False; 3];
    for (i, &v) in Ternary::all().iter().enumerate() {
        table[i] = unary_op(op, v, system);
    }
    table
}

/// Check entailment: does `a` entail `b` in the given system?
/// Entailment holds when implication(a, b) is designated (True).
pub fn entails(a: Ternary, b: Ternary, system: LogicSystem) -> bool {
    binary_op(BinaryOp::Implication, a, b, system) == Ternary::True
}

/// Check if a unary formula is a tautology (always True for all inputs).
pub fn is_tautology_unary<F>(f: F, system: LogicSystem) -> bool
where
    F: Fn(Ternary, LogicSystem) -> Ternary,
{
    Ternary::all().iter().all(|&v| f(v, system) == Ternary::True)
}

/// Check if a binary formula is a tautology.
pub fn is_tautology_binary<F>(f: F, system: LogicSystem) -> bool
where
    F: Fn(Ternary, Ternary, LogicSystem) -> Ternary,
{
    Ternary::all()
        .iter()
        .all(|&a| Ternary::all().iter().all(|&b| f(a, b, system) == Ternary::True))
}

/// A propositional formula in ternary logic.
#[derive(Clone, Debug, PartialEq)]
pub enum Formula {
    Atom(Ternary),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Equiv(Box<Formula>, Box<Formula>),
    Necessity(Box<Formula>),
    Possibility(Box<Formula>),
}

impl Formula {
    /// Evaluate the formula under a given logic system.
    pub fn eval(&self, system: LogicSystem) -> Ternary {
        match self {
            Formula::Atom(v) => *v,
            Formula::Not(inner) => negate(inner.eval(system), system),
            Formula::And(l, r) => binary_op(BinaryOp::Conjunction, l.eval(system), r.eval(system), system),
            Formula::Or(l, r) => binary_op(BinaryOp::Disjunction, l.eval(system), r.eval(system), system),
            Formula::Implies(l, r) => binary_op(BinaryOp::Implication, l.eval(system), r.eval(system), system),
            Formula::Equiv(l, r) => binary_op(BinaryOp::Equivalence, l.eval(system), r.eval(system), system),
            Formula::Necessity(inner) => necessity(inner.eval(system), system),
            Formula::Possibility(inner) => possibility(inner.eval(system), system),
        }
    }
}

/// Count the number of designated (True) entries in a binary truth table.
pub fn count_designated_binary(table: &[[Ternary; 3]; 3]) -> usize {
    table.iter().flat_map(|row| row.iter()).filter(|&&v| v == Ternary::True).count()
}

/// Count the number of Unknown entries in a binary truth table.
pub fn count_unknown_binary(table: &[[Ternary; 3]; 3]) -> usize {
    table.iter().flat_map(|row| row.iter()).filter(|&&v| v == Ternary::Unknown).count()
}

/// Designated values for a logic system (values that count as "true enough").
pub fn designated_values(system: LogicSystem) -> Vec<Ternary> {
    match system {
        LogicSystem::Kleene | LogicSystem::Lukasiewicz | LogicSystem::GödelDummett => {
            vec![Ternary::True]
        }
        LogicSystem::Bochvar => vec![Ternary::True],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ternary_to_u8_roundtrip() {
        for &v in Ternary::all() {
            assert_eq!(Ternary::from_u8(v.to_u8()), Some(v));
        }
        assert_eq!(Ternary::from_u8(3), None);
    }

    #[test]
    fn test_negation_double() {
        for &v in Ternary::all() {
            let neg = negate(v, LogicSystem::Kleene);
            let negneg = negate(neg, LogicSystem::Kleene);
            assert_eq!(v, negneg, "double negation should be identity for {:?}", v);
        }
    }

    #[test]
    fn test_kleene_conjunction() {
        assert_eq!(kleene_conj(Ternary::True, Ternary::True), Ternary::True);
        assert_eq!(kleene_conj(Ternary::True, Ternary::False), Ternary::False);
        assert_eq!(kleene_conj(Ternary::True, Ternary::Unknown), Ternary::Unknown);
        assert_eq!(kleene_conj(Ternary::False, Ternary::Unknown), Ternary::False);
        assert_eq!(kleene_conj(Ternary::Unknown, Ternary::Unknown), Ternary::Unknown);
    }

    #[test]
    fn test_kleene_disjunction() {
        assert_eq!(kleene_disj(Ternary::False, Ternary::False), Ternary::False);
        assert_eq!(kleene_disj(Ternary::True, Ternary::False), Ternary::True);
        assert_eq!(kleene_disj(Ternary::False, Ternary::Unknown), Ternary::Unknown);
        assert_eq!(kleene_disj(Ternary::True, Ternary::Unknown), Ternary::True);
    }

    #[test]
    fn test_lukasiewicz_implication() {
        assert_eq!(lukasiewicz_impl(Ternary::True, Ternary::True), Ternary::True);
        assert_eq!(lukasiewicz_impl(Ternary::True, Ternary::False), Ternary::False);
        assert_eq!(lukasiewicz_impl(Ternary::Unknown, Ternary::Unknown), Ternary::True);
        assert_eq!(lukasiewicz_impl(Ternary::False, Ternary::False), Ternary::True);
    }

    #[test]
    fn test_bochvar_unknown_propagation() {
        assert_eq!(bochvar_conj(Ternary::True, Ternary::Unknown), Ternary::Unknown);
        assert_eq!(bochvar_disj(Ternary::False, Ternary::Unknown), Ternary::Unknown);
        let impl_val = binary_op(BinaryOp::Implication, Ternary::True, Ternary::Unknown, LogicSystem::Bochvar);
        assert_eq!(impl_val, Ternary::Unknown);
    }

    #[test]
    fn test_godel_conjunction_min() {
        assert_eq!(godel_conj(Ternary::True, Ternary::False), Ternary::False);
        assert_eq!(godel_conj(Ternary::True, Ternary::Unknown), Ternary::Unknown);
        assert_eq!(godel_conj(Ternary::False, Ternary::False), Ternary::False);
    }

    #[test]
    fn test_godel_implication() {
        assert_eq!(godel_impl(Ternary::True, Ternary::False), Ternary::False);
        assert_eq!(godel_impl(Ternary::False, Ternary::Unknown), Ternary::True);
        assert_eq!(godel_impl(Ternary::Unknown, Ternary::Unknown), Ternary::True);
    }

    #[test]
    fn test_truth_table_binary_size() {
        let table = truth_table_binary(BinaryOp::Conjunction, LogicSystem::Kleene);
        // 3x3 = 9 entries
        assert_eq!(table.len(), 3);
        assert_eq!(table[0].len(), 3);
    }

    #[test]
    fn test_truth_table_unary_size() {
        let table = truth_table_unary(UnaryOp::Negation, LogicSystem::Kleene);
        assert_eq!(table.len(), 3);
    }

    #[test]
    fn test_entailment_kleene() {
        // True entails True
        assert!(entails(Ternary::True, Ternary::True, LogicSystem::Kleene));
        // False entails everything in Kleene
        assert!(entails(Ternary::False, Ternary::False, LogicSystem::Kleene));
        assert!(entails(Ternary::False, Ternary::True, LogicSystem::Kleene));
        // True does not entail False
        assert!(!entails(Ternary::True, Ternary::False, LogicSystem::Kleene));
    }

    #[test]
    fn test_tautology_excluded_middle_lukasiewicz() {
        // L3: p OR NOT p is a tautology? Let's check
        let is_taut = is_tautology_binary(|a, b, sys| {
            let _ = b;
            binary_op(BinaryOp::Disjunction, a, negate(a, sys), sys)
        }, LogicSystem::Lukasiewicz);
        // In L3, Unknown OR NOT Unknown = Unknown, so NOT a tautology
        assert!(!is_taut);
    }

    #[test]
    fn test_identity_tautology() {
        // a IMPLIES a should be a tautology in L3
        let is_taut = is_tautology_binary(|a, b, sys| {
            let _ = b;
            binary_op(BinaryOp::Implication, a, a, sys)
        }, LogicSystem::Lukasiewicz);
        assert!(is_taut);
    }

    #[test]
    fn test_formula_eval_simple() {
        let f = Formula::Not(Box::new(Formula::Atom(Ternary::True)));
        assert_eq!(f.eval(LogicSystem::Kleene), Ternary::False);
    }

    #[test]
    fn test_formula_eval_conjunction() {
        let f = Formula::And(
            Box::new(Formula::Atom(Ternary::True)),
            Box::new(Formula::Atom(Ternary::Unknown)),
        );
        assert_eq!(f.eval(LogicSystem::Kleene), Ternary::Unknown);
    }

    #[test]
    fn test_formula_necessity() {
        let f = Formula::Necessity(Box::new(Formula::Atom(Ternary::True)));
        assert_eq!(f.eval(LogicSystem::Kleene), Ternary::True);
        let f2 = Formula::Necessity(Box::new(Formula::Atom(Ternary::Unknown)));
        assert_eq!(f2.eval(LogicSystem::Kleene), Ternary::False);
    }

    #[test]
    fn test_formula_possibility() {
        let f = Formula::Possibility(Box::new(Formula::Atom(Ternary::Unknown)));
        assert_eq!(f.eval(LogicSystem::Kleene), Ternary::True);
    }

    #[test]
    fn test_count_designated_kleene_conjunction() {
        let table = truth_table_binary(BinaryOp::Conjunction, LogicSystem::Kleene);
        let count = count_designated_binary(&table);
        // Only True AND True = True => 1 designated
        assert_eq!(count, 1);
    }

    #[test]
    fn test_count_unknown_kleene_conjunction() {
        let table = truth_table_binary(BinaryOp::Conjunction, LogicSystem::Kleene);
        let count = count_unknown_binary(&table);
        // True&U, U&True, U&U, False&U skip (False wins), U&False skip
        // Actually: True&U=U, U&True=U, U&U=U => 3 unknowns
        assert_eq!(count, 3);
    }

    #[test]
    fn test_ternary_display() {
        assert_eq!(format!("{}", Ternary::False), "F");
        assert_eq!(format!("{}", Ternary::Unknown), "U");
        assert_eq!(format!("{}", Ternary::True), "T");
    }

    #[test]
    fn test_is_classical() {
        assert!(Ternary::True.is_classical());
        assert!(Ternary::False.is_classical());
        assert!(!Ternary::Unknown.is_classical());
    }

    #[test]
    fn test_designated_values() {
        for sys in [LogicSystem::Kleene, LogicSystem::Lukasiewicz, LogicSystem::Bochvar, LogicSystem::GödelDummett] {
            let dv = designated_values(sys);
            assert!(dv.contains(&Ternary::True));
        }
    }
}
