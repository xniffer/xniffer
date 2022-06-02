<div id="top"></div>
<div align="center">
	<a href="https://github.com/3top1a/xniffer/stargazers"><img alt="GitHub stars" src="https://img.shields.io/github/stars/3top1a/xniffer?style=for-the-badge"></a>
	<a href="https://github.com/3top1a/xniffer/network"><img alt="GitHub forks" src="https://img.shields.io/github/forks/3top1a/xniffer?style=for-the-badge"></a>
	<a href="https://github.com/3top1a/xniffer/issues"><img alt="GitHub issues" src="https://img.shields.io/github/issues/3top1a/xniffer?style=for-the-badge"></a>
	<a href="https://github.com/3top1a/xniffer/blob/main/LICENSE"><img alt="GitHub license" src="https://img.shields.io/github/license/3top1a/xniffer?style=for-the-badge"></a>
	<a href="https://github.com/3top1a/xniffer/actions/workflows/rust.yml"><img alt="GitHub actions" src="https://github.com/3top1a/xniffer/actions/workflows/rust.yml/badge.svg"></a>
	<h1><code>metaxata</code></h1>
	<h2>A library combining multiple metadata crates into one</h2>
</div>

## About

Metaxata is a library for metadata, meant to combine multiple crates and libraries into one.

## Workflow

Metaxata returns a vector of the Data struct.
The data struct looks like the following:

```rust
struct Data {
	tag: String,
	value: Value,
	provider: Provider,
}
```

Metaxata works on tag-value pairs, with a provider to indicate the library used.
The value enum is used to indicate the type, such as a string or a float.

```rust
enum Value
{
	String(String),
	Integer(i64),
	Number(f64),
	Raw(Vec<u8>),
	Time(u64)
}
```

## Built With

- id3
- lofty
- rexif
- kamadak-exif

## Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request.
Don't forget to give the project a star! Thanks again!

## License
Distributed under the AGPL-3.0 License. See `LICENSE` for more information.

## Contact
If you want more stuff like this, [I have a website!](https://3top1a.github.io/)
