// Eventual automatic type level currying, need associated types + multidispatch.
#![feature(associated_types, macro_rules, unboxed_closures, unboxed_closure_sugar)]

trait Curry {
    type Res;
    fn curry(self) -> <Self as Curry>::Res;
}

impl<Arg1, Arg2, R, F, G, H> Curry for F
    where F : Fn<(Arg1, Arg2), R>,
          G : Fn<Arg1, H>,
          H : Fn<Arg2, R> {
    type Res = G;
    fn curry(self) -> G {
        fail!("foo")
    }
}

fn main() {
    try_curry(|&: x: int, y: int| x + y);
}

fn try_curry<F: Fn<(int, int), int> + Curry>(fun: F) -> <F as Curry>::Res {
    fun.curry()
}
