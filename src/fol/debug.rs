use super::*;
use std::fmt::{Debug, Formatter, Result};

impl Debug for Term {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match *self {
            Term::Var(v) => write!(fmt, "{}", v),
            Term::Fn(ref f) => write!(fmt, "{:?}", f),
        }
    }
}

impl Debug for Apply {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let &Apply { name, ref terms } = self;
        write!(fmt, "{}(", name)?;
        for t in Iterator::intersperse(terms.iter().map(Some), None) {
            if let Some(term) = t {
                write!(fmt, "{:?}", term)?;
            } else {
                write!(fmt, ", ")?;
            }
        }
        write!(fmt, ")")
    }
}
