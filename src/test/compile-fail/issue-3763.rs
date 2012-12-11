// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-test
mod my_mod {
    pub struct MyStruct {
        priv priv_field: int
    }
    pub fn MyStruct () -> MyStruct {
        MyStruct {priv_field: 4}
    }
}

fn main() {
    let my_struct = my_mod::MyStruct();
    let _woohoo = (&my_struct).priv_field; // compiles but shouldn't
    let _woohoo = (~my_struct).priv_field; // ditto
    let _woohoo = (@my_struct).priv_field; // ditto
   // let nope = my_struct.priv_field;       // compile error as expected
}