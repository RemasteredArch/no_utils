# no_utils

Rust rewrites of various programs, usually coreutils, and other small Rust programs.

## Utils

no_utils is just a container. All programs are in their own workspaces, under `utils/`.

- [rat](./utils/rat/): cat(1), but worse.
    - Real alternative: [bat](https://github.com/sharkdp/bat).
- [onebrc](./utils/onebrc/): a Rust implementation of Gunnar Morling's [One Billion Row Challenge](https://www.morling.dev/blog/one-billion-row-challenge/).
- [fortune_bot](./utils/fortune_bot/): a Discord bot to serve fortunes from [fortune(6)](https://en.wikipedia.org/wiki/Fortune_(Unix)).

## License

no_utils is licensed under the GNU Affero General Public License version 3, or (at your option) any later version. You should have received a copy of the GNU Affero General Public License along with no_utils, found in [LICENSE](./LICENSE). If not, see <[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.
