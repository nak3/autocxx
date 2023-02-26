// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::PathBuf;
fn main() -> miette::Result<()> {
    let mut b = autocxx_build::Builder::new(
        "src/lib.rs",
        &PathBuf::new(),
    )
    .build()?;

    b.flag_if_supported("-std=c++14").compile("llvm");
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
