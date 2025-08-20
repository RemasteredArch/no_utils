# Noise

An extremely simple noise pattern generator.

Outputs an image file in the [PPM] format.

## Usage

```
$ noise [options] [--] {width} {height} [hex [hex ...]]
```

Where `hex` is a hexadecimal RGB value, in the form of `#RRGGBB` or `RRGGBB`,
and `width` and `height` are non-zero integer values.

Not including hex values will use truly random RGB values.
If hex values are given, it will randomly choose one of the given colors per pixel.
Repeat a color to make it show up more often.

Options:

- `-h`/`--help`: Displays a help message.
- `-o`/`--output` `{path}`:
  Write to a file instead of `STDOUT`.

## License

Noise is a part of no_utils.
no_utils is licensed under the GNU Affero General Public License version 3, or (at your option) any later version.
You should have received a copy of the GNU Affero General Public License along with no_utils, found in [`../../LICENSE`](../../LICENSE).
If not, see <https://www.gnu.org/licenses/>.

[PPM]: <https://en.wikipedia.org/wiki/Netpbm#PPM_example>
