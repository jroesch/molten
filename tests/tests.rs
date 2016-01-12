#![feature(type_macros)]
#[macro_use] extern crate molten;

use molten::generic::Generic;

struct Point {
    x: i32,
    y: i32
}

impl Generic for Point {
    type Repr = HList!(i32,i32);
}

fn test_generic_point() {
    let point = Point { x: 0, y: 0 };
}
