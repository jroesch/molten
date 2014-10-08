///  Simple encoding of type equality for fully saturated types.
trait TypeEq<A> {}
impl<A> TypeEq<A> for A {}
