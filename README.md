# Remnant: From the Ashes Item Checklist

An offline enabled web app for tracking what items you've unlocked in
[Gunfire Games's](https://gunfiregames.com/) [Remnant: From the Ashes](https://www.remnantgame.com).

While the application is dual licensed MIT and Apache, all game related assets
and content are © Gunfire Games, LLC.

The application is built with the [Yew](yew.rs/) framework, which compiles Rust
to WASM. It's built with [wasm-pack](https://github.com/rustwasm/wasm-pack) and ❤️.

## Data Out of Date?

Data for the application is stored in CSV files in the `src/data/` directory.
Please file a pull requests or an issue if you find issues with the data.

## Development

While not all parts are necessarily needed, easy mode development requires:

- [install Rust](https://rustup.rs/)
- [install Docker](https://docs.docker.com/get-docker/)
- install a `Procfile` runner, like [forego](https://github.com/ddollar/forego)

If you're on a mac and have [Homebrew](https://docs.brew.sh/Installation) installed,
you can run `brew install` to install `forego`.

To start development, run `forego start` which will kick off the `Procfile` that:

- watches files to automatically run tests
- rebuilds the wasm file when tests pass
- runs a local webserver using static files and Docker

## Testing

Locally, you can run `bin/lint.sh` to lint the source files and `bin/test.sh`
to run the Rust and WASM related tests.

The application is setup to run linters and tests in Github Actions.

## Production

Running `bin/build.sh` creates a production release of static files in the
`public/` directory.

The application is hosted on [Netlify](https://www.netlify.com/) and is setup
to automatically publish CI builds that pass. It does not automatically take
new builds live in production so they can be tested before final release.
