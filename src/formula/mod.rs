#![allow(dead_code)]

use lalrpop_intern::InternedString;
use std::collections::BTreeSet;
use std::sync::Arc;

#[macro_use]
mod macros;
mod debug;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub struct Formula<T> {
    pub kind: Arc<FormulaKind<T>>
}

impl<T> Formula<T> {
    pub fn is_false(&self) -> bool {
        match *self.kind {
            FormulaKind::False => true,
            _ => false
        }
    }

    pub fn is_true(&self) -> bool {
        match *self.kind {
            FormulaKind::True => true,
            _ => false
        }
    }

    /// Returns true if this is a `(not ..)` formula.
    ///
    /// `negative` in the book
    pub fn is_negative(&self) -> bool {
        match *self.kind {
            FormulaKind::Not(_) => true,
            _ => false
        }
    }

    /// Returns true if this is not a `(not ..)` formula. Useful for
    /// literal formulas (e.g., p, (not p)) and not so much for more
    /// complex ones.
    ///
    /// `positive` in the book
    pub fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    /// If this is a positive formula, returns `(not {self})`.
    /// Otherwise, if this is a negative formula, removes the
    /// negation.
    ///
    /// Note the subtle distinction from `not()`, which always wraps
    /// in a negation.
    ///
    /// Intended for use only on literal formulas.
    ///
    /// `negate` in the book.
    pub fn negate(&self) -> Formula<T> {
        match *self.kind {
            FormulaKind::Not(ref p) => p.clone(),
            _ => self.not(),
        }
    }

    /// Returns `(not self)`.
    pub fn not(&self) -> Formula<T> {
        formula!(not {self})
    }

    pub fn with(kind: FormulaKind<T>) -> Self {
        Formula { kind: Arc::new(kind) }
    }

    /// Maps atoms to other atoms.
    ///
    /// In the book, `onatoms`.
    pub fn on_atoms<U>(&self, f: &mut dyn FnMut(&T) -> Formula<U>) -> Formula<U> {
        match *self.kind {
            FormulaKind::False => Formula::with(FormulaKind::False),
            FormulaKind::True => Formula::with(FormulaKind::True),
            FormulaKind::Atom(ref a) => f(a),
            FormulaKind::Not(ref a) => Formula::with(FormulaKind::Not(a.on_atoms(f))),
            FormulaKind::Or(ref a, ref b) => Formula::with(FormulaKind::Or(a.on_atoms(f), b.on_atoms(f))),
            FormulaKind::And(ref a, ref b) => Formula::with(FormulaKind::And(a.on_atoms(f), b.on_atoms(f))),
            FormulaKind::Implies(ref a, ref b) => Formula::with(FormulaKind::Implies(a.on_atoms(f), b.on_atoms(f))),
            FormulaKind::Iff(ref a, ref b) => Formula::with(FormulaKind::Iff(a.on_atoms(f), b.on_atoms(f))),
            FormulaKind::ForAll(n, ref b) => Formula::with(FormulaKind::ForAll(n, b.on_atoms(f))),
            FormulaKind::Exists(n, ref b) => Formula::with(FormulaKind::Exists(n, b.on_atoms(f))),
        }
    }

    /// Processes atoms sequentially, folding some state through.
    ///
    /// In the book, `overatoms`.
    pub fn over_atoms<U>(&self, u: U, f: &mut dyn FnMut(&T, U) -> U) -> U {
        match *self.kind {
            FormulaKind::False => u,
            FormulaKind::True => u,

            FormulaKind::Atom(ref a) => f(a, u),

            FormulaKind::ForAll(_, ref a) |
            FormulaKind::Exists(_, ref a) |
            FormulaKind::Not(ref a) => a.over_atoms(u, f),

            FormulaKind::Or(ref a, ref b) |
            FormulaKind::And(ref a, ref b) |
            FormulaKind::Implies(ref a, ref b) |
            FormulaKind::Iff(ref a, ref b) => a.over_atoms(b.over_atoms(u, f), f),
        }
    }

    /// Applies `f()` to each atom, then unions together the results into a set.
    ///
    /// In the book, `atom_union`.
    pub fn atom_union<U: Ord>(&self, f: &mut dyn FnMut(&T) -> U) -> BTreeSet<U> {
        self.over_atoms(
            BTreeSet::new(),
            &mut |atom, mut set| {
                set.insert(f(atom));
                set
            })
    }

    /// Convert a formula into a vector of conjunctions.
    ///
    /// In the book, `conjunctions`.
    pub fn conjuncts(&self) -> Vec<&Formula<T>> {
        let mut v = vec![];
        let mut p = self;
        loop {
            if let FormulaKind::And(ref a, ref b) = *p.kind {
                v.push(a);
                p = b;
            } else {
                v.push(p);
                break;
            }
        }
        v
    }
}

impl<T> Clone for Formula<T> {
    fn clone(&self) -> Self {
        Formula { kind: self.kind.clone() }
    }
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum FormulaKind<T> {
    False,
    True,
    Atom(T),
    Not(Formula<T>),
    And(Formula<T>, Formula<T>),
    Or(Formula<T>, Formula<T>),
    Implies(Formula<T>, Formula<T>),
    Iff(Formula<T>, Formula<T>),
    ForAll(InternedString, Formula<T>),
    Exists(InternedString, Formula<T>),
}

impl<T> FormulaKind<T> {
    /// Extract the two sides of an `iff`.
    ///
    /// In the book, `dest_iff`.
    pub fn iff(&self) -> Option<(&Formula<T>, &Formula<T>)> {
        match *self {
            FormulaKind::Iff(ref a, ref b) => Some((a, b)),
            _ => None,
        }
    }

    /// Extract the two sides of an `and`.
    ///
    /// In the book, `dest_and`.
    pub fn and(&self) -> Option<(&Formula<T>, &Formula<T>)> {
        match *self {
            FormulaKind::And(ref a, ref b) => Some((a, b)),
            _ => None,
        }
    }
}

