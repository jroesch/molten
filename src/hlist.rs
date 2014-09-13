use std::fmt::{Formatter, Show, FormatError};

trait HList {
  fn cons<H>(self, xs: H) -> HCons<H, Self>;
}

struct HCons<H, T> {
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
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
    formatter.write_str("HNil"); Ok(())
  }
}

impl<H: Show, T: HList + Show> Show for HCons<H, T> {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
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

fn main() {
  let xs: HCons<int, HCons<int, HNil>> = hnil().cons(2).cons(1);
  //hnil().head(); //fails
  println!("{}", xs);
  let ys = hnil().cons(1u);
  println!("{}", ys.head());
}
