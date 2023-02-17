# Dacti Project

This repository is a sandbox workspace for building the "dacti" metaverse object format.

Dacti itself refers to the "dacti package" and "dacti object" specification.
It is a work-in-progress object format for describing the visual representation of data, and
attaching functionality to that data, in an and extensible way that's easy to support at various
levels.

The dacti object speficiation can be used for objects 'at rest'.
For example, when stored on a file system, or sent to applications as a file.
The object specifications can be used both to describe the entirety of an object, and to describe a
subset of an object, or a replicated mirror of an object for a client to display.

The dacti package speficiation can bundle a collection of data, which can be indexed and
subdivided with simple offsets.
This package format allows you to bundle an object specification together with its data in one
file, to store and transfer objects in a self-contained way.
As well, the offset indexes allow you to bundle a large collection of objects and data in a file,
that can be fetched sparsely using [HTTP range requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests).

The name "dacti" comes from the lojban word describing something as being a material object
enduring in space-time.

Everything in this workspace is not ready for production use.
All APIs, file formats, and other details are subject to change at any time.

## Daicon

Daicon containers are a wrapping file format, made to make file self-description and versioning easier. They let a file format describe itself using a UUID and semantic version. Additionally, they provide a flexible way to define named and versioned regions of data in the file, called "interfaces".

For more information, see the [draft daicon specification](docs/daicon.md).

## Pterodactil

Pterodactil is a CLI toolkit for working with dacti formats.

## Dacti viewer

To explore the development of this format, this repository contains a reference viewer
implementation.
This viewer may be eventually split out and renamed to become its own application in the future.

## Building Standalone JS Library

This project uses wasm-bindgen to create the JS bindings around the WebAssembly based package.
To generate the package for use in projects, run `wasm-pack build` in the JS crate's directory.
See the [wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/) for more
information.

You do not need to build the library manually for the web example.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (Expat) ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
