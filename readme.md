# Colony

[![CI](https://github.com/AlexAegis/colony/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/colony/actions/workflows/ci.yml)

A game experiment using Bevy.

> Try out the web version at <https://alexaegis.github.io/colony/>

## Resources

- <https://bevyengine.org/learn/book/getting-started/setup/>
- <https://github.com/Anshorei/awesome-bevy>
- <https://rust-lang.github.io/mdBook/format/summary.html>
- <https://bfnightly.bracketproductions.com/chapter_33.html>

## Requirements

- [Latest Rust Stable](https://rustup.rs/)
- wasm32-unknown-uwknown compilation target

  > For web target

  ```sh
  rustup target add wasm32-unknown-unknown
  ```

- [Trunk](https://trunkrs.dev/)

  > For web target

  ```sh
  cargo install --locked trunk
  ```

## Usage

Running the default game

```sh
cargo run
```

Developing web assets

```sh
cd ./apps/colony-client/
trunk serve
```

Building for the web (When served from the root of the repo, for example when
using [Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer))

```sh
cd ./apps/colony-client/
trunk build --public-url target/web/
```
