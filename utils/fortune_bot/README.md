# fortune_bot

Wisdom of Unix past, brought straight to your server.

## Usage

- `/help`
- `/fortune`

### Future Options:

- `/license [full]`

## Setup

Requires a `.env` file in the project root (`utils/fortune_bot/.env`):

```sh
TOKEN=YOUR_TOKEN # Discord bot token
GUILD_ID=YOUR_TESTING_GUILD_ID # ID of the guild you will use to test the bot
```

Also requires [`fortune`](<https://en.wikipedia.org/wiki/Fortune_(Unix)>) to be in PATH,
such that it can be run with
`cmd /C fortune` (Windows) or `sh -c fortune` (otherwise).

## License

rat is a part of no_utils. no_utils is licensed under the GNU Affero General Public License version 3, or (at your option) any later version. You should have received a copy of the GNU Affero General Public License along with no_utils, found in [../../LICENSE](../../LICENSE). If not, see \<[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.
