# Dacti Project

Dacti is a modular format for describing metaverse objects and packaging metaverse data.

The name "dacti" comes from the lojban word describing something as being a material object enduring in space-time.

[Read the dacti specification draft here!](docs/index.md)

## Dacti Objects

Dacti objects are a flexible self-describing metaverse object format, based on daicon components.

The dacti objects can be used for objects 'at rest'. For example, when stored on a file system, or sent to applications. The object specification can be used to describe the entirety of an object, and to describe a subset of an object, or a replicated mirror of an object for a client to display.

A system supporting dacti objects should support any binary blob of data as an object. Dacti object format allows these binary blobs to describe how they should be interpreted when supported.

## Dacti Packages

The dacti package speficiation can bundle a collection of data, which can be indexed and subdivided with simple offsets. This package format allows you to bundle an object specification together with its data in one file, to store and transfer objects in a self-contained way. As well, the offset indexes allow you to bundle a large collection of objects and data in a file, that can be fetched sparsely using [HTTP range requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests).

## Related Projects

- [Daicon](https://github.com/open-mv-sandbox/daicon): Dacti uses daicon as the base for its binary formats.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (Expat) ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
