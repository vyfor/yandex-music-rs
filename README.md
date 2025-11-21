# yandex-music-rs [![crates badge]][crates.io] [![docs badge]][docs.rs]
[crates badge]: https://img.shields.io/crates/v/yandex-music
[crates.io]: https://crates.io/crates/yandex-music
[docs badge]: https://img.shields.io/badge/docs.rs-rustdoc-green
[docs.rs]: https://docs.rs/yandex-music

A wrapper around the [Yandex Music API](https://music.yandex.ru) in Rust. Made exclusively for educational purposes only. Not affiliated with Yandex in any way.

## Installation

```bash
cargo add yandex-music
```

## Usage

```rust
use yandex_music::YandexMusicClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Replace "TOKEN" with your Yandex Music access token
	let client = YandexMusicClient::builder("TOKEN").build()?;

	// Example usage
	let status = client.get_account_status().await?;
	println!("Account status: {status:?}");

	Ok(())
}
```

> [!NOTE]
> You may also want to take a look at [yamusic](https://github.com/yamusic/yamusic).

## Acknowledgements
- https://github.com/acherkashin/yandex-music-open-api
- https://github.com/MarshalX/yandex-music-api