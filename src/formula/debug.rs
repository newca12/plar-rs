use super::*;
use std::fmt::{Debug, Error, Formatter};

impl<T: Debug> Debug for Formula<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{:?}", self.kind)
    }
}

impl<T: Debug> Debug for FormulaKind<T> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            FormulaKind::False => write!(fmt, "false"),
            FormulaKind::True => write!(fmt, "true"),
            FormulaKind::Atom(ref t) => write!(fmt, "[{:?}]", t),
            FormulaKind::Not(ref t) => write!(fmt, "(not {:?})", t),
            FormulaKind::And(ref a, ref b) => write!(fmt, "(and {:?} {:?})", a, b),
            FormulaKind::Or(ref a, ref b) => write!(fmt, "(or {:?} {:?})", a, b),
            FormulaKind::Implies(ref a, ref b) => write!(fmt, "(implies {:?} {:?})", a, b),
            FormulaKind::Iff(ref a, ref b) => write!(fmt, "(iff {:?} {:?})", a, b),
            FormulaKind::ForAll(n, ref b) => write!(fmt, "(forall<{}> {:?})", n, b),
            FormulaKind::Exists(n, ref b) => write!(fmt, "(exists<{}> {:?})", n, b),
        }
    }
}
