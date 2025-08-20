// SPDX-License-Identifier: AGPL-3.0-or-later
//
// Copyright © 2025 RemasteredArch
//
// This file is part of noise. Noise is a part of no_utils.
//
// no_utils is free software: you can redistribute it and/or modify it under the terms of the GNU
// Affero General Public License as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// no_utils is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License along with no_utils. If
// not, see <https://www.gnu.org/licenses/>.

use std::fs::File;
use std::io::{BufWriter, Write, stdout};
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::{env, io};

mod image;
mod noise;

use image::{Image, Palette, Rgb};
use noise::{Noise, PaletteNoiseGenerator, RandomNoiseGenerator};

fn main() -> io::Result<()> {
    let mut args = env::args_os().skip(1); // Ignore the execution path.

    let mut parse_args = true;
    let mut path: Option<PathBuf> = None;
    let mut palette = Vec::new();

    let mut width: Option<NonZeroUsize> = None;
    let mut height: Option<NonZeroUsize> = None;

    while let Some(arg) = args.next() {
        let arg = arg.to_str().expect("Error: encountered invalid UTF-8");
        let next_hex;

        if parse_args {
            match arg {
                "--" => {
                    parse_args = false;

                    continue;
                }
                "-h" | "--help" => {
                    help();

                    return Ok(());
                }
                "-o" | "--output" => {
                    path = Some(
                        args.next()
                            .and_then(|p| p.to_str().map(ToString::to_string))
                            .expect("Error: expected path after --output")
                            .into(),
                    );

                    continue;
                }
                arg if width.is_none() => {
                    width = Some(
                        NonZeroUsize::new(
                            arg.parse().unwrap_or_else(|_| {
                                panic!("Error: unrecognized argument {arg}, expected image width in number of pixels")
                            })
                        ).expect("Error: expected non-zero width value")
                    );

                    continue;
                }
                arg if height.is_none() => {
                    height = Some(
                        NonZeroUsize::new(
                            arg.parse().unwrap_or_else(|_| {
                                panic!("Error: unrecognized argument {arg}, expected image height in number of pixels")
                            })
                        ).expect("Error: expected non-zero height value")
                    );

                    continue;
                }
                arg => next_hex = arg,
            }
        } else {
            next_hex = arg;
        }

        palette.push(
            Rgb::from_hex(next_hex).unwrap_or_else(|| {
                panic!("Error: unrecognized argument {arg}, expected hex color")
            }),
        )
    }

    let mut output: Box<dyn Write>;
    if let Some(path) = path {
        let writer = BufWriter::new(File::create(path)?);
        output = Box::new(writer);
    } else {
        output = Box::new(stdout().lock());
    }

    let width = width.expect("Error: expected image width in number of pixels");
    let height = height.expect("Error: expected image height in number of pixels");

    if !palette.is_empty() {
        let palette = Palette::new(palette.into_boxed_slice()).unwrap();
        let generator = PaletteNoiseGenerator::new(palette);
        Noise::new(width, height, generator)
            .to_ppm(&mut output)
            .unwrap();
    } else {
        let generator = RandomNoiseGenerator::new();
        Noise::new(width, height, generator)
            .to_ppm(&mut output)
            .unwrap();
    }

    Ok(())
}

fn help() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    const LICENSE: &str = env!("CARGO_PKG_LICENSE");
    let help_message: String = format!(
        "{NAME} v{VERSION}

Licensed under the terms of the {LICENSE}.
(C) {AUTHORS}.

Usage: ./{NAME} [options] [--] {{width}} {{height}} [hex [hex ...]]

Where `hex` is a hexadecimal RGB value, in the form of `#RRGGBB` or `RRGGBB`,
and `width` and `height` are non-zero integer values.

Not including hex values will use truly random RGB values. If hex values are
given, it will randomly choose one of the given colors per pixel. Repeat a
color to make it show up more often.

Outputs an image file in the PPM format.
See <https://en.wikipedia.org/wiki/Netpbm#PPM_example>.

Options:
  -h | --help               Prints this help message
  -o | --output {{path}}    Write to a file instead of STDOUT"
    );

    println!("{help_message}");
}
