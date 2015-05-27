use std::collections::HashSet;
use std::hash::Hash;
use std::ops::{Fn, FnMut, FnOnce};

trait PolyFn {
    fn apply<A>(&self, x: A) -> Self::Result where Self : Case<A> {
        self.at(x)
    }
}

trait Case<A> {
    type Result;
    fn at(&self, x: A) -> Self::Result;
}

#[allow(non_camel_case_types)]
struct singleton;

impl PolyFn for singleton {}

impl<T: Eq + Hash> Case<(T,)> for singleton {
    type Result = HashSet<T>;

    fn at(&self, x: (T,)) -> HashSet<T> {
        let mut hs = HashSet::new();
        hs.insert(x.0); hs
    }
}

// pub trait Fn<Args>: FnMut<Args> {
//     extern "rust-call" fn call(&self, args: Args) -> Self::Output;
// }
//
// pub trait FnMut<Args>: FnOnce<Args> {
//     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
// }

impl<Args> FnOnce<Args> for singleton where singleton : Case<Args> {
    type Output = <Self as Case<Args>>::Result;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        singleton.apply(args)
    }
}

#[allow(non_camel_case_types)]
struct identity;
impl PolyFn for identity {}

impl<T> Case<T> for identity {
    type Result = T;
    fn at(&self, x: T) -> T { x }
}

#[allow(non_camel_case_types)]
struct headOption;

#[allow(non_camel_case_types)]
struct size;

#[test]
fn test_singleton() {
    let x = singleton(10);
    let y = singleton("hello");
}
//
// macro_rules! poly {
//     ($name:ident, $($case:item)+) => (struct $name; $($case)+);
// }
//
// macro_rules! case {
//     ($name:ident, $arg_ty:ty, $result_ty:ty, $body:item) => (
//         impl Case<$arg_ty> for $name {
//             type Result = $result_ty;
//             fn (&self, x: A) -> $result_ty { panic!() }
//         }
//     )
// }
//
// poly!(length,
//     case!(length, i32, 4);
//     case!(length, String, 4););
