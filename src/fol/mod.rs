use itertools::Itertools;
use lalrpop_intern::InternedString;
use std::collections::HashMap;
use std::iter::{empty, once};
use std::fmt::Debug;
use std::io::Write;
use util::{IteratorObject, Substitution};

mod debug;
mod test;

#[derive(Clone, PartialEq, Eq)]
pub enum Term {
    Var(InternedString),
    Fn(Apply),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Apply {
    name: InternedString,
    terms: Vec<Term>
}

pub struct Domain {
    constant_terms: Vec<Term>,
    funcs: HashMap<InternedString, usize>,
}

impl Domain {
    pub fn ground_terms(&self, n: usize) -> IteratorObject<Term> {
        if n == 0 {
            IteratorObject::new(self.constant_terms.iter().cloned())
        } else {
            IteratorObject::new(
                self.funcs
                    .iter()
                    .flat_map(move |(&name, &arity)| {
                        self.ground_tuples(n - 1, arity)
                            .map(move |terms| Term::Fn(Apply { name, terms }))
                    }))
        }
    }

    pub fn ground_tuples(&self, n: usize, arity: usize) -> IteratorObject<Vec<Term>> {
        if arity == 0 {
            if n == 0 {
                IteratorObject::new(once(vec![]))
            } else {
                IteratorObject::new(empty())
            }
        } else {
            IteratorObject::new(
                (0...n)
                    .flat_map(move |k| {
                        self.ground_terms(k)
                            .cartesian_product(self.ground_tuples(n - k, arity - 1)
                                               .collect::<Vec<_>>())
                            .map(move |(e, mut v)| {
                                v.push(e);
                                v
                            })
                    }))
        }
    }
}

struct HerbrandLoop<'d, F>
    where F: Clone + Debug,
{
    domain: &'d Domain,
    modification_fn: Box<Fn(&Vec<F>, &Substitution<InternedString, Term>, &Vec<F>) -> Vec<F>>,
    testing_fn: Box<Fn(&Vec<F>) -> bool>,
    initial_formula: Vec<F>,
    free_variables: Vec<InternedString>,
    out: &'d mut Write,
}

impl<'d, F> HerbrandLoop<'d, F>
    where F: Clone + Debug
{
    fn execute(&mut self, formula: Vec<F>) -> Vec<Vec<Term>> {
        self.execute1(0, formula, vec![], IteratorObject::new(empty()))
    }
    fn execute1(&mut self,
                mut n: usize,
                mut formula: Vec<F>,
                mut tried: Vec<Vec<Term>>,
                mut tuples: IteratorObject<'d, Vec<Term>>)
                -> Vec<Vec<Term>> {
        loop {
            write!(self.out, "{} ground instances tried, ", tried.len()).unwrap();
            write!(self.out, "{} items in list\n", formula.len()).unwrap();
            match tuples.next() {
                None => {
                    tuples = self.domain.ground_tuples(n, self.free_variables.len());
                    n += 1;
                }

                Some(tuple) => {
                    let subst = Substitution::new(&self.free_variables, &tuple);
                    let formula1 = (self.modification_fn)(&self.initial_formula, &subst, &formula);
                    tried.push(tuple);
                    if !(self.testing_fn)(&formula1) {
                        return tried;
                    } else {
                        formula = formula1;
                    }
                }
            }
        }
    }
}

// pick up here, page 160
// fn gilmore_loop(domain: &Domain) {
// }
