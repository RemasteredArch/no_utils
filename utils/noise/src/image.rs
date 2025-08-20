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

use std::{io::Write, num::NonZeroUsize};

pub type Value = u8;

#[derive(Copy, Clone)]
pub struct Rgb {
    pub red: Value,
    pub green: Value,
    pub blue: Value,
}

impl Rgb {
    /// Creates a [`Self`] from a hexadecimal value like `#RRGGBB` or `RRGGBB`.
    pub fn from_hex(hex: &str) -> Option<Self> {
        // Split into chunks of two hex digits each.
        let chars = hex.trim_start_matches('#').chars().collect::<Box<[_]>>();
        let mut chunks = chars.chunks_exact(2);

        // Collect each chunk of digits into a string, then parse that into a [`u8`].
        let mut parse_chunk =
            || u8::from_str_radix(chunks.next()?.iter().collect::<String>().as_str(), 16).ok();

        Some(Self {
            red: parse_chunk()?,
            green: parse_chunk()?,
            blue: parse_chunk()?,
        })
    }
}

#[derive(Clone)]
pub struct Palette {
    colors: Box<[Rgb]>,
}

impl Palette {
    pub fn new(colors: Box<[Rgb]>) -> Option<Self> {
        if colors.is_empty() {
            return None;
        }

        Some(Self { colors })
    }

    pub const fn colors(&self) -> &[Rgb] {
        &self.colors
    }
}

/// Represents an image.
///
/// # Safety
///
/// Unsafe to implement because of the requirements that the lengths of [`Self::rows`] match
/// [`Self::width`] and [`Self::height`].
pub unsafe trait Image {
    /// Returns the width of the image, in number of pixels.
    fn width(&self) -> NonZeroUsize;

    /// Returns the height of the image, in number of pixels.
    fn height(&self) -> NonZeroUsize;

    /// Returns as [`Self::height`] rows of exactly length [`Self::width`].
    ///
    /// As a result, the length must be exactly [`Self::width`] times [`Self::height`].
    fn rows(&mut self) -> impl Iterator<Item = impl Iterator<Item = Rgb>>;

    /// Writes [`Self`] as a [PPM] file.
    ///
    /// [PPM]: <https://en.wikipedia.org/wiki/Netpbm#PPM_example>
    fn to_ppm(&mut self, output: &mut impl Write) -> std::io::Result<()> {
        writeln!(
            output,
            "P3 {} {} {}",
            self.width(),
            self.height(),
            Value::MAX
        )?;

        for mut row in self.rows() {
            // Write the first pixel first, to avoid starting with a space.
            //
            // Not required by the format, but it makes me happy.
            let Rgb { red, green, blue } =
                row.next().expect("rows must be at least one pixel long");
            write!(output, "{red} {green} {blue}")?;

            for Rgb { red, green, blue } in row {
                write!(output, " {red} {green} {blue}")?;
            }

            // Write a newline after every row.
            //
            // Not required by the format, but it makes me happy.
            writeln!(output)?;
        }

        Ok(())
    }
}
