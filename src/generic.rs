use std::mem::transmute;

pub trait Generic : Sized {
    type Repr;

    fn as_repr(&self) -> &Self::Repr {
        unsafe { transmute(self) }
    }
}
