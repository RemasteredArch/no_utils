// SPDX-License-Identifier: AGPL-3.0-or-later
//
// Copyright © 2024 RemasteredArch
//
// This file is part of onebrc. onebrc is a part of no_utils.
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

#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let args = env::args_os().skip(1); // Skip execution path

    let mut parse_args = true;

    for arg in args {
        if parse_args && (arg == "-h" || arg == "--help") {
            help();

            return Ok(());
        }

        if parse_args && arg == "--" {
            parse_args = false;

            continue;
        }

        let path: &Path = Path::new(&arg);

        let exists: Result<bool, std::io::Error> = path.try_exists();

        match exists {
            Ok(true) => print_file(path)?,
            Ok(false) => eprintln!("Error: {} does not exist!", path.to_string_lossy()),
            Err(error) => eprintln!("{}", error),
        }
    }

    Ok(())
}

fn print_file(path: &Path) -> io::Result<()> {
    let file = File::open(path).expect("To be able to read the file.");
    let reader = BufReader::new(file);

    let mut stations = HashMap::<String, Station>::new();

    for line in reader.lines() {
        let result = line.unwrap();

        add_to_stations(&mut stations, parse_line(&result));
    }

    print_stations(&stations);

    Ok(())
}

fn print_stations(stations: &HashMap<String, Station>) {
    let stations = BTreeMap::from_iter(stations.iter());
    let mut stations = stations.iter();

    let (name, station) = stations.next().unwrap();
    print!(
        "{{{}={}/{}/{}",
        name,
        station.get_min(),
        station.get_average(),
        station.get_max(),
    );

    for (name, station) in stations {
        print!(
            ", {}={}/{}/{}",
            name,
            station.get_min(),
            station.get_average(),
            station.get_max()
        );
    }

    println!("}}");
}

fn add_to_stations(stations: &mut HashMap<String, Station>, measurement: Measurement) {
    if let Some(station) = stations.get_mut(measurement.name) {
        station.total += measurement.value;
        station.count += 1;

        station.min = std::cmp::min(station.min, measurement.value as i32);
        station.max = std::cmp::max(station.max, measurement.value as i32);
    } else {
        stations.insert(
            measurement.name.to_string(),
            Station::new_from_entry(measurement),
        );
    }
}

struct Measurement<'st> {
    name: &'st str,
    value: i64,
}

impl<'st> Measurement<'st> {
    const fn new_from_data(name: &'st str, value: i64) -> Self {
        Self { name, value }
    }
}

#[derive(Debug)]
struct Station {
    total: i64,
    count: i32,
    min: i32,
    max: i32,
}

impl Station {
    const fn new() -> Self {
        Self {
            total: 0,
            count: 0,
            min: 0,
            max: 0,
        }
    }

    fn new_from_entry(measurement: Measurement) -> Self {
        Self {
            total: measurement.value,
            count: 1,
            min: measurement.value as i32,
            max: measurement.value as i32,
        }
    }

    fn get_min(&self) -> f32 {
        self.min as f32 / 10.0
    }

    fn get_average(&self) -> f32 {
        (self.total / self.count as i64) as f32 / 10.0
    }

    fn get_max(&self) -> f32 {
        self.max as f32 / 10.0
    }
}

fn parse_line(line: &str) -> Measurement {
    let (name, temperature_str) = line.split_once(';').unwrap();
    let temperature = temperature_str.replacen('.', "", 1).parse::<i64>().unwrap();

    Measurement::new_from_data(name, temperature)
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

Usage: ./{NAME} [options] [--] {{path [path ...]}}

Options:
  -h | --help     Prints this help message"
    );

    println!("{}", help_message);
}
