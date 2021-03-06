<div id="top"></div>
<div align="center">
	<a href="https://github.com/xniffer/xniffer/stargazers"><img alt="GitHub stars" src="https://img.shields.io/github/stars/xniffer/xniffer?style=for-the-badge"></a>
	<a href="https://github.com/xniffer/xniffer/network"><img alt="GitHub forks" src="https://img.shields.io/github/forks/xniffer/xniffer?style=for-the-badge"></a>
	<a href="https://github.com/xniffer/xniffer/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues/xniffer/xniffer?style=for-the-badge"></a>
	<a href="https://github.com/xniffer/xniffer/blob/main/LICENSE"><img alt="GitHub license" src="https://img.shields.io/github/license/xniffer/xniffer?style=for-the-badge"></a>
	<a href="https://github.com/xniffer/xniffer/actions/workflows/cli.yml"><img alt="GitHub actions" src="https://github.com/xniffer/xniffer/actions/workflows/cli.yml/badge.svg"></a>
	<h1><code>📃 xniffer</code></h1>
	<h2>A simple exif sniffer written in Rust</h2>
</div>

## About

Xniffer is a simple EXIF reader/sniffer built with [Rust](https://www.rust-lang.org/).
It's meant to be a simple learning and performance exercise for me.

Metaxata is a library for metadata, meant to combine multiple metadata crates and libraries into one.

## Built With
I tried using the least amount of dependencies, so that the code is simple.

- [rayon](https://github.com/rayon-rs/rayon)
- [comfy-table](https://github.com/Nukesor/comfy-table)
- [termsize](https://github.com/softprops/termsize)

### Prerequisites
[Install rust](https://www.rust-lang.org/tools/install) and clone this repo

### Installation
```bash
git clone https://github.com/xniffer/xniffer.git
cd xniffer
cargo build --release
```

The executable will be under `target/release`

## Usage
```bash
./target/release/xniffer ~/Desktop/file.png
# Xniffer also supports folders.
./target/release/xniffer ~/Desktop
```

## Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request.
Don't forget to give the project a star! Thanks again!

## License
Distributed under the AGPL-3.0 License. See `LICENSE` for more information.

## Contact
If you want more stuff like this, [I have a website!](https://3top1a.github.io/) It ain't much, but it's functional
