#![allow(incomplete_features, non_camel_case_types)]
#![feature(specialization)]

mod math;
use math::*;

// (a + b) * (b + c) = a*b + a*c + b*b + b*c
fn h1<A: Variable, B: Variable>() {}
    
// (a + b) * (b + c) * (c + a) + a*b*c = (a + b + c) * (a*b + b*c + c*a)
fn pairs_mul_add_triple_eq_sum_mul_pairs<A: Variable, B: Variable, C: Variable>() {
}
