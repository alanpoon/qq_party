// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation for WASI
use crate::Error;
extern crate std;
use std::io::Write;
use crate::host_call::host_call;
pub fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    //NumberGen.GenerateGuid
    // let m :[u8;0]=[]; 
    // let mut buf = host_call("default","wasmcloud:builtin:numbergen","GenerateGuid",&m).unwrap();
    // buf.write(dest);
    Ok(())
}
