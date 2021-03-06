// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
fn main () {
    let x = 4;
    match x {
        ref r if *r < 0 => println!("got negative num {} < 0", r),
        e @ 1 ..= 100 => println!("got number within range [1,100] {}", e),
        _ => println!("no"),
    }
}
