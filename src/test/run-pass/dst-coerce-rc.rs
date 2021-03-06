// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test a very simple custom DST coercion.

#![feature(core, rc_weak)]

use std::cell::RefCell;
use std::rc::{Rc, Weak};

trait Baz {
    fn get(&self) -> i32;
}

impl Baz for i32 {
    fn get(&self) -> i32 {
        *self
    }
}

fn main() {
    let a: Rc<[i32; 3]> = Rc::new([1, 2, 3]);
    let b: Rc<[i32]> = a;
    assert_eq!(b[0], 1);
    assert_eq!(b[1], 2);
    assert_eq!(b[2], 3);

    let a: Rc<i32> = Rc::new(42);
    let b: Rc<Baz> = a.clone();
    assert_eq!(b.get(), 42);

    let c: Weak<i32> = a.downgrade();
    let d: Weak<Baz> = c.clone();

    let _c = b.clone();

    let a: Rc<RefCell<i32>> = Rc::new(RefCell::new(42));
    let b: Rc<RefCell<Baz>> = a.clone();
    assert_eq!(b.borrow().get(), 42);
    let c: Weak<RefCell<Baz>> = a.downgrade();
}
