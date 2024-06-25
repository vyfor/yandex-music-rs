# yandex-music-rs [![crates badge]][crates.io] [![docs badge]][docs.rs]
[crates badge]: https://img.shields.io/crates/v/yandex-music
[crates.io]: https://crates.io/crates/yandex-music
[docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-green
[docs.rs]: https://docs.rs/yandex-music

A wrapper around the [Yandex Music API](https://music.yandex.ru) in Rust. Not affiliated with Yandex in any way.

## Installation

```bash
cargo add yandex-music
```

## Usage

```rust
use yandex_music::YandexMusicClient;

let client = YandexMusicClient::new("TOKEN");
```

## Acknowledgements
- https://github.com/acherkashin/yandex-music-open-api
- https://github.com/MarshalX/yandex-music-api