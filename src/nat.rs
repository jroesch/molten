#![feature(associated_types)]
use std::fmt;
use std::fmt::{Formatter, Show};

// I tried to do associated types but this causes bugs should investigate,
// and submit a patch.

pub trait Nat {
    //type Repr;
    //fn value() -> <Self as Nat>::Repr;
    fn to_int(&self) -> int;
    // fn materialize_nat<N: Nat>() -> N
}

pub struct Z;
pub struct S<N: Nat> { pred: N }

impl Nat for Z {
    // type Repr = Z;
    // fn value() -> <Z as Nat>::Repr { Z }
    fn to_int(&self) -> int { 0 }
}

impl Show for Z {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        let _ = fmt.write_str("0");
        Ok(())
    }
}



impl<N: Nat> Nat for S<N> {
    fn to_int(&self) -> int {
        1 + self.pred.to_int()
    }
}

impl<N: Nat> Show for S<N> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        let _ = fmt.write_str("0");
        Ok(())
    }
}

trait LessThan<N: Nat>: Nat {
    fn less_than(&self, n: N) -> bool { true }
}

impl<N: Nat> LessThan<S<N>> for Z {}
impl<N: Nat, M: Nat> LessThan<S<N>> for S<M> where M: LessThan<N> {}

// trait SizedVec<N: Nat, A, R> {
//     fn new(vec: Vec<A>) -> Result<SVec<N, A>, String>;
//     fn index<I: Nat>(i: I) -> A;
// }
//
// impl<A> SizedVec<Z, A> for SVec<Z, A> {
//     fn new(vec: Vec<A>) -> Result<SVec<Z, A>, String> {
//         if vec.len() == 0 {
//             Ok(SVec { vec: vec })
//         } else {
//             Err("fail".to_string())
//         }
//     }
//
//     fn index<I: Nat>(i: I) -> A { panic!("yolo"); }
// }
//
// impl<N: Nat, A> SizedVec<S<N>, A> for SVec<S<N>, A> {
//     fn new(vec: Vec<A>) -> Result<SVec<S<N>, A>, String> {
//         if vec.len() > 0 {
//             Ok(SVec { vec: vec })
//         } else {
//             Err("fail".to_string())
//         }
//     }
//
//     fn index<I: Nat>(i: I) -> A { panic!("yolo"); }
// }
//
//
// struct SVec<N: Nat, A> {
//     vec: Vec<A>
// }
//
// type three = S<S<S<Z>>>;
//
// #[test]
// fn simple_test() {
//     // Nat
//     let s = S(S(Z));
//     s.less_than(Z);
//     println!("{}", s.to_int());
//     // Sized
//     let sz: SVec<three, int> = SizedVec::new(vec![1i, 2i, 3i]).unwrap();
//     //println!("Value at Index: {}", sz.index()
// }
