use hlist::*;
use generic::*;

// macro for implementing n-ary tuple functions and operations
macro_rules! generic_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
        )+) => {
        $(
            impl<$($T),+> Generic for ($($T,)+) {
                type Repr = HList![$($T),+];
            }
            )+
    }

}

generic_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
            (1) -> B
    }
    Tuple3 {
        (0) -> A
            (1) -> B
            (2) -> C
    }
    Tuple4 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
    }
    Tuple5 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
            (4) -> E
    }
    Tuple6 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
            (4) -> E
            (5) -> F
    }
    Tuple7 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
            (4) -> E
            (5) -> F
            (6) -> G
    }
    Tuple8 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
            (4) -> E
            (5) -> F
            (6) -> G
            (7) -> H
    }
    Tuple9 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
            (4) -> E
            (5) -> F
            (6) -> G
            (7) -> H
            (8) -> I
    }
    Tuple10 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
            (4) -> E
            (5) -> F
            (6) -> G
            (7) -> H
            (8) -> I
            (9) -> J
    }
    Tuple11 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
            (4) -> E
            (5) -> F
            (6) -> G
            (7) -> H
            (8) -> I
            (9) -> J
            (10) -> K
    }
    Tuple12 {
        (0) -> A
            (1) -> B
            (2) -> C
            (3) -> D
            (4) -> E
            (5) -> F
            (6) -> G
            (7) -> H
            (8) -> I
            (9) -> J
            (10) -> K
            (11) -> L
    }
}
