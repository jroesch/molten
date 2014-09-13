use std::fmt::{Formatter, Show, FormatError};

trait Nat {}

struct Z;
struct Succ<N> {
  pred: Box<N>
}

impl Nat for Z {}
impl<N: Nat> Nat for Succ<N> {}

impl Show for Z {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
    formatter.write_str("0"); Ok(())
  }
}

impl<N: Nat + Show> Show for Succ<N> {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), FormatError> {
    formatter.write_str(format!("1 + {}", self.pred.to_string()).as_slice()) ; Ok(())
  }
}

trait LessThan<M> {
  fn less_than(&self, m: M) -> bool;
}

impl<M: Nat> LessThan<Succ<M>> for Z {
  fn less_than(&self, m: Succ<M>) -> bool { true }
}

impl<M: Nat, N: Nat + LessThan<M>> LessThan<Succ<M>> for Succ<N> {
  fn less_than(&self, m: Succ<M>) -> bool { true }
}
