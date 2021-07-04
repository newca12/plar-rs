#![cfg_attr(not(test), allow(dead_code))]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(iter_intersperse)]

extern crate itertools;
extern crate lalrpop_intern;

#[macro_use]
mod formula;
mod fol;
mod prop;
mod util;

#[test]
fn it_works() {}
