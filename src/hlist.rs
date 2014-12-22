use std::fmt;
use std::fmt::{Formatter, Show};
use nat::*;

trait HList {
  fn cons<H>(self, xs: H) -> HCons<H, Self>;
}

struct HCons<H, T: HList> {
  head: H,
  tail: Box<T>
}

struct HNil;

impl<S, T: HList> HList for HCons<S, T> {
  fn cons<H>(self, h: H) -> HCons<H, HCons<S, T>> {
    HCons { head: h, tail: box self }
  }
}

impl HList for HNil {
  fn cons<H>(self, h: H) -> HCons<H, HNil> {
    HCons { head: h, tail: box self }
  }
}

fn hnil() -> HNil { HNil }

impl Show for HNil {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
    formatter.write_str("HNil"); Ok(())
  }
}

impl<H: Show, T: HList + Show> Show for HCons<H, T> {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
    formatter.write_str(self.head.to_string().as_slice());
    formatter.write_str(" :: ");
    formatter.write_str(self.tail.to_string().as_slice());
    Ok(())
  }
}

// This is where we want associated types, really inconv. to specify proof
// relations
trait IsHCons<H, T> {
  fn head(&self) -> &H;
  fn tail(&self) -> &T;
}

impl<H, T: HList> IsHCons<H, T> for HCons<H, T> {
  fn head(&self) -> &H {
    &self.head
  }

  fn tail(&self) -> &T {
    &*self.tail
  }
}

// Still not stable enough
trait Take<N: Nat> {
    type R: HList;
    fn take(&self, n: N) -> <Self as Take<N>>::R;
}

impl<H: HList> Take<Z> for H {
    type R = H;
    fn take(&self, n: Z) -> H {
        panic!("I only care about type checking")
    }
}

// FIXME (#18655) refers to the error here.
impl<H, T: HList + Take<N>, N: Nat> Take<S<N>> for HCons<H, T> {
    type R = HCons<H, <T as Take<N>>::R>;
    fn take(&self, n: S<N>) -> HCons<H, <T as Take<N>>::R> {
        panic!("only care about type checking")
    }
}

// // This part doesn't work w/o multi dispatch
// trait Take<N, H, R> {
//   fn take(&self, n: N) -> R;
// }
//
// impl<H: HList> Take<Z, H, H> for H {
//   fn take(self, n: Z) -> H {
//     self
//   }
// }
//
// impl<N: Nat, H, T: HList + Take<N, T, R>, R: HList> Take<Succ<N>, HCons<H, T>, R> for HCons<H, T> {
//   fn take(self, n: Succ<N>) -> R {
//     self.tail.take(*n.pred)
//   }
// }

#[test]
fn test_that_take_works() {
    let xs: HCons<int, HCons<int, HNil>> = hnil().cons(2).cons(1);
    let ys = xs.take(Z);
    let zs = xs.take(S { pred: Z });
}
