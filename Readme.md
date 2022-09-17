# m3u8_downloader_rs

This is an m3u8 downloader developed with Rust.

It adopts tokio and reqwest to send http requests concurrently.

Currently it is not designed for encrypted m3u8.

Here is a [m3u8](http://1257120875.vod2.myqcloud.com/0ef121cdvodtransgzp1257120875/3055695e5285890780828799271/v.f230.m3u8) file demonstration.

Usage:

cargo run -- -u m3u8_url

cargo run -- --url m3u8_url

./m3u8_downloader_rs -u m3u8_url

./m3u8_downloader_rs --url m3u8_url

---

v1 @ 2022.09.18: Barely usable.