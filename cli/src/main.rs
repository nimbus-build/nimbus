// Copyright (c) 2026 The Nimbus Authors. All rights reserved.
//
// The use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Nimbus CLI")]
struct Cli {}

fn main() -> Result<()> {
    let _cli = Cli::parse();
    println!("Hello, world!");
    Ok(())
}
