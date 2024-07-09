// SPDX-License-Identifier: AGPL-3.0-or-later
//
// Copyright © 2024 RemasteredArch
//
// This file is part of rat. rat is a part of no_utils.
//
// no_utils is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// no_utils is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License along with no_utils. If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let args = env::args_os().skip(1); // Skip execution path

    for arg in args {
        let path: &Path = Path::new(&arg);

        let exists: Result<bool, std::io::Error> = path.try_exists();

        match exists {
            Ok(true) => print_file(path),
            Ok(false) => println!("Error: {} does not exist!", path.to_string_lossy()),
            Err(error) => println!("{}", error),
        }
    }
}

fn print_file(path: &Path) {
    let file = File::open(path).expect("To be able to read the file.");
    let reader = BufReader::new(file);

    let mut buffer = String::new();
    let mut buffer_length: u8 = 0;

    for line in reader.lines() {
        buffer.push_str(&line.unwrap());
        buffer.push('\n');
        buffer_length += 1;

        if buffer_length == u8::MAX {
            print!("{}", buffer);
        }
    }
}
