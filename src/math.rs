pub trait Polynomial: Copy + 'static {}

pub trait Term: Copy + 'static {}

pub trait Variable: Copy + 'static {}

#[derive(Clone, Copy)]
pub struct Mul<L: Term, R: Term> {
    left: L,
    right: R,
}

#[derive(Clone, Copy)]
pub struct Add<L: Term, R: Term> {
    left: L,
    right: R,
}

#[derive(Clone, Copy)]
pub struct Parentheses<P: Polynomial> {
    term: P,
}

impl<T: Term> Polynomial for T {}
impl<L: Term, R: Term> Polynomial for Add<L, R> {}

impl<V: Variable> Term for V {}
impl<L: Term, R: Term> Term for Mul<L, R> {}
impl<P: Polynomial> Term for Parentheses<P> {}

pub trait EquivalentTo<P: Polynomial, const ID: usize>: Polynomial {}

// reflexivity
impl<P: Polynomial> EquivalentTo<P, 0> for P {}

// symmetricity
impl<P1: Polynomial, P2: Polynomial> EquivalentTo<P2, 1> for P1
where P2: EquivalentTo<P1, 2> {}

// transitivity
/*
[can't compile]
the type parameter P1 is not constrained by the impl trait, self type, or predicates
unconstrained type parameter (rustc E0207)]

impl<P1: Polynomial, P2: Polynomial, P3: Polynomial> EquivalentTo<P3, 3> for P2 where
    P1: EquivalentTo<P3, 4>,
    P2: EquivalentTo<P1, 5>,
{}
*/

// R1 = R2  -> L + R1 = L + R2
impl<L: Term, R1: Term, R2: Term>
EquivalentTo<Add<L, R2>, 6> for Add<L, R1>
where
    R1: EquivalentTo<R2, 7>,
{}

// L1 = L2  -> L1 + R = L2 + R
impl<L1: Term, R: Term, L2: Term>
EquivalentTo<Add<L2, R>, 8> for Add<L1, R>
where
    L1: EquivalentTo<L2, 9>,
{}

// R1 = R2  -> L * R1 = L * R2
impl<L: Term, R1: Term, R2: Term>
EquivalentTo<Mul<L, R2>, 10> for Mul<L, R1>
where
    R1: EquivalentTo<R2, 11>,
{}

// L1 = L2  -> L1 * R = L2 * R
impl<L1: Term, R: Term, L2: Term>
EquivalentTo<Mul<L2, R>, 12> for Mul<L1, R>
where
    L1: EquivalentTo<L2, 13>,
{}

// // a + b = b + a
// impl<A: Term, B: Term>
// EquivalentTo<Add<B, A>, 14> for Add<A, B>
// {}
pub trait add_comm { type to: Polynomial; }
impl<P: Polynomial> add_comm for P {
    default type to = P;
}
impl<A: Term, B: Term> add_comm for Add<A, B> {
    type to = Add<B, A>;
}
pub fn add_comm

// a * b = b * a
impl<A: Term, B: Term>
EquivalentTo<Mul<B, A>, 15> for Mul<A, B>
{}

// (a + b) + c  = a + (b + c)
impl<A: Term, B: Term, C: Term>
EquivalentTo<Add<Parentheses<Add<A, B>>, C>, 16> for Add<A, Parentheses<Add<B, C>>>
{}

// (a * b) * c  = a * (b * c)
impl<A: Term, B: Term, C: Term>
EquivalentTo<Mul<Parentheses<Mul<A, B>>, C>, 17> for Mul<A, Parentheses<Mul<B, C>>>
{}

// a * (b + c) = a*b + a*c
impl<A: Term, B: Term, C: Term>
EquivalentTo<Mul<A, Parentheses<Add<B, C>>>, 18> for Add<Mul<A, B>, Mul<A, C>>
{}

// (a + b) * c = a*c + b*c
impl<A: Term, B: Term, C: Term>
EquivalentTo<Mul<Parentheses<Add<A, B>>, C>, 19> for Add<Mul<A, C>, Mul<B, C>>
{}
