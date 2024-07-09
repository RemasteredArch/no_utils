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

use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::{env, io};

fn main() -> io::Result<()> {
    let args = env::args_os().skip(1); // Skip execution path

    for arg in args {
        let path: &Path = Path::new(&arg);

        let exists: Result<bool, std::io::Error> = path.try_exists();

        match exists {
            Ok(true) => print_file(path)?,
            Ok(false) => println!("Error: {} does not exist!", path.to_string_lossy()),
            Err(error) => println!("{}", error),
        }
    }

    Ok(())
}

fn print_file(path: &Path) -> io::Result<()> {
    const BUFFER_SIZE: usize = u16::MAX as usize;
    let mut stdout = std::io::stdout().lock();

    let mut file = File::open(path).expect("To be able to read the file.");
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    let file_length = file.metadata()?.len();
    let mut cursor: u64 = 0;

    while cursor <= file_length {
        if file.read_exact(&mut buffer).is_err() {
            break;
        }
        stdout.write_all(&buffer)?;
        cursor = file.stream_position()?;
    }

    // Brings the seek head/cursor position back to before the last buffer was read
    let start_of_last_buffer: i64 = (cursor as i128 - file_length as i128) as i64;
    file.seek(io::SeekFrom::End(start_of_last_buffer))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    stdout.write_all(&buffer)?;

    Ok(())
}
