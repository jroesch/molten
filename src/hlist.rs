use std::fmt::{self, Debug};

pub struct HNil;
pub struct HCons<H, T: HList>(H, T);

pub trait HList {}

impl HList for HNil {}
impl<H, T: HList> HList for HCons<H, T> {}

impl Debug for HNil {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_str("HNil")
    }
}

impl<H: Debug, T: HList + Debug> Debug for HCons<H, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(self.0.fmt(fmt)); self.1.fmt(fmt)
    }
}

#[macro_export]
macro_rules! hlist({ } => { $crate::hlist::Nil } ; { $ head : expr } => {
                   $crate::hlist::HCons ( $ head , $crate::hlist::HNil ) } ; {
                   $ head : expr , $ ( $ tail : expr ) , * } => {
                   $crate::hlist::HCons ( $ head , hlist ! ( $ ( $ tail ) , * ) ) } ;);

#[macro_export]
macro_rules! HList({  } => { $crate::hlist::HNil } ; { $ head : ty } => {
                   $crate::hlist::HCons < $ head , $crate::hlist::HNil > } ; {
                   $ head : ty , $ ( $ tail : ty ) , * } => {
                   $crate::hlist::HCons < $ head , HList ! ( $ ( $ tail ) , * ) > } ;);
