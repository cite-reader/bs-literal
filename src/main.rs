/*
   Copyright 2016 Alex Hill

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */

extern crate clap;

use std::ascii;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Read, Write};

use clap::{Arg, App};

fn main() {
    let args = App::new("Bytestring Literals")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Alex Hill <alexander.d.hill.89@gmail.com>")
        .about("Escapes binary into a form suitable for a Rust literal")
        .arg(Arg::with_name("input").help("The input file to escape (STDIN if not given)"))
        .get_matches();

    if let Some(filename) = args.value_of_os("input") {
        go(BufReader::new(File::open(filename).unwrap()));
    } else {
        let i = stdin();
        go(i.lock());
    }
}

fn go<R: Read>(input: R) {
    let out = stdout();
    let mut writer = out.lock();

    for byte in input.bytes()
        .flat_map(|b| ascii::escape_default(b.unwrap())) {
        writer.write_all(&[byte]).unwrap();
    }

    writer.write_all(b"\n").unwrap();
}
