# OMF Object Viewer Prototype

Prototype viewer for metaverse objects, for both native and browser platforms.

This is a sandbox workspace for explorations in building the Open Metaverse Foundation object
specification, and tools for creating and viewing these objects.
This is not ready for production use, all APIs, file formats, and other details are subject to
change at any time.

## Building Standalone JS Library

This project uses wasm-bindgen to create the JS bindings around the WebAssembly based package.
To generate the package for use in projects, run `wasm-pack build` in the JS crate's directory.
See the [wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/) for more
information.

You do not need to build the library manually for the web example.
