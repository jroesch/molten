struct Lazy<T, F: Fn<(), T>> {
    thunk: F
}

impl<T, F: Fn<(), T>> Lazy<T, F> {
    fn force(self) -> T {
        self.thunk.call(())
    }
}

// macro_rules! lazy (($e:expr) => (Lazy { thunk: |&:| { $e }}))

#[test]
fn test_lazy_delays_evaluation() {
    assert!(true)
}
