# {{project-name}}

## Dependencies
* Rust
* Trunk `cargo install --locked trunk`
* `wasm32-unknown-unknown` target `rustup target add wasm32-unknown-unknown`
* Tailwind CSS `npm install -g tailwindcss-cli`

## How to build
### Debug
* `make dist`
### Release
* `make release`

## How to run
### Debug
* `make run`
### Release
* `make run_release`