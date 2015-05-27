#[derive(Debug)]
struct HNil;

#[derive(Debug)]
struct HCons<T, H>(T, H) where H: HList;

trait HList {}

impl HList for HNil {}

impl<T, H> HList for HCons<T, H> where H: HList {}

#[test]
fn test_construct_hlist() {
    let hlist = HCons(1, HCons("hello", HNil));
}
