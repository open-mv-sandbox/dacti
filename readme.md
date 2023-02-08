# OMF Object Viewer Prototype

Prototype to explore a possible 'self-describing object' file type, for development under the Open Metaverse Foundation.
This project currently has no endorsement from the OMF, it is merely an experimental prototype.
All APIs and details are subject to change at any time.

## Building Standalone JS Library

This project uses wasm-bindgen to create the JS bindings around the WASM code.
To generate a package you can bundle, run `wasm-pack build` in the JS crate's directory.
See the [wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/) for more information.
The web example does not use wasm-pack (directly), rather delegating it to a bundler.
