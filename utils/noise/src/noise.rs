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

use std::{cell::RefCell, num::NonZeroUsize, rc::Rc};

use rand::{RngCore, SeedableRng, rngs::SmallRng};

use crate::image::{Image, Palette, Rgb};

pub trait NoiseGenerator {
    fn next(&mut self) -> Rgb;
}

#[derive(Clone)]
pub struct RandomNoiseGenerator {
    rng: SmallRng,
}

impl RandomNoiseGenerator {
    pub fn new() -> Self {
        Self::from_rng(SmallRng::from_rng(&mut rand::rng()))
    }

    pub fn from_rng(rng: SmallRng) -> Self {
        Self { rng }
    }

    fn next_u8(&mut self) -> u8 {
        (self.rng.next_u32() % u8::MAX as u32) as u8
    }
}

impl NoiseGenerator for RandomNoiseGenerator {
    fn next(&mut self) -> Rgb {
        Rgb {
            red: self.next_u8(),
            green: self.next_u8(),
            blue: self.next_u8(),
        }
    }
}

#[derive(Clone)]
pub struct PaletteNoiseGenerator {
    rng: SmallRng,
    palette: Palette,
}

impl PaletteNoiseGenerator {
    pub fn new(palette: Palette) -> Self {
        Self::from_rng(SmallRng::from_rng(&mut rand::rng()), palette)
    }

    pub fn from_rng(rng: SmallRng, palette: Palette) -> Self {
        Self { rng, palette }
    }

    fn next_usize(&mut self) -> usize {
        #[cfg(target_pointer_width = "32")]
        return self.rng.next_u32() as usize;

        #[cfg(target_pointer_width = "64")]
        return self.rng.next_u64() as usize;
    }
}

impl NoiseGenerator for PaletteNoiseGenerator {
    fn next(&mut self) -> Rgb {
        let rand = self.next_usize();
        let colors = self.palette.colors();

        // Will not panic because `colors.len()` must be greater than zero.
        let index = rand % colors.len();
        // Will not panic because the index will be no greater than the last index.
        colors[index]
    }
}
#[derive(Clone)]
pub struct NoiseRow<G: NoiseGenerator> {
    columns_left: usize,
    generator: Rc<RefCell<G>>,
}

impl<G: NoiseGenerator> Iterator for NoiseRow<G> {
    type Item = Rgb;

    fn next(&mut self) -> Option<Self::Item> {
        if self.columns_left == 0 {
            return None;
        }

        self.columns_left -= 1;
        Some(self.generator.borrow_mut().next())
    }
}

pub struct Noise<G: NoiseGenerator> {
    width: NonZeroUsize,
    height: NonZeroUsize,
    rows_left: usize,
    generator: Rc<RefCell<G>>,
}

impl<G: NoiseGenerator> Noise<G> {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize, generator: G) -> Self {
        Self {
            width,
            height,
            rows_left: height.get(),
            generator: Rc::new(RefCell::new(generator)),
        }
    }
}

unsafe impl<G: NoiseGenerator + Clone> Image for Noise<G> {
    fn width(&self) -> NonZeroUsize {
        self.width
    }

    fn height(&self) -> NonZeroUsize {
        self.height
    }

    fn rows(&mut self) -> impl Iterator<Item = impl Iterator<Item = Rgb>> {
        self
    }
}

impl<G> Iterator for Noise<G>
where
    G: NoiseGenerator + Clone,
{
    type Item = NoiseRow<G>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rows_left == 0 {
            return None;
        }

        self.rows_left -= 1;
        Some(NoiseRow {
            columns_left: self.width.get(),
            generator: self.generator.clone(),
        })
    }
}
