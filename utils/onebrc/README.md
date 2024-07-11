# onebrc

A Rust solution to Gunnar Morling's [One Billion Row Challenge](https://www.morling.dev/blog/one-billion-row-challenge/).

Written as a coding challenge for me, not as a genuine submission, so rules shall not apply.

## Running

```sh
$ ./scripts/create_measurements.sh billion # Or --help for more details
$ cargo run -- measurements_1000000000.txt # Not yet actually implemented
```
*The generator requires OpenJDK 21.*

## License

onebrc is a part of no_utils. no_utils is licensed under the GNU Affero General Public License version 3, or (at your option) any later version. You should have received a copy of the GNU Affero General Public License along with no_utils, found in [../../LICENSE](../../LICENSE). If not, see <[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.

### Dependency Licenses

onebrc contains software originally developed by Gunnar Morling and the One Billion Row Challenge contributors, licensed under the Apache License, Version 2.0. You may obtain a copy of this license at <[http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0)>.

The following files contain code from the One Billion Row Challenge, licensed under the terms of the Apache License, Version 2.0:
- [`scripts/dev/morling/onebrc/CreateMeasurements.java`](./scripts/dev/morling/onebrc/CreateMeasurements.java)
