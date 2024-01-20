<p align="center">
    <h1 align="center">GT_GRAPH_WASM</h1>
</p>
<p align="center">
    <em>WASM & Rust reader for the gt file format</em>
</p>
<p align="center">
	<img src="https://img.shields.io/github/license/semohr/gt_graph_wasm?style=flat&color=0080ff" alt="license">
	<img src="https://img.shields.io/github/last-commit/semohr/gt_graph_wasm?style=flat&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=Rust&logoColor=white" alt="Rust">
	<img src="https://img.shields.io/badge/Webpack-8DD6F9.svg?style=flat&logo=Webpack&logoColor=black" alt="Webpack">
<p>

<hr>

---
## About

This is a WebAssembly module for loading graphs in the [gt file format](https://graph-tool.skewed.de/static/doc/gt_format.html). It is written in Rust and compiled to WebAssembly using [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/). The module is then bundled into a JavaScript module such that it can be used in JavaScript and TypeScript projects.

At the moment only loading graphs is supported. Writing capabilities might be added in the future but are not a priority for me as this library was developed to allow visualizing gt graphs in the browser.


##  Repository Structure

```sh
└── gt_graph_wasm/
    ├── Cargo.toml
    ├── index.js
    ├── package-lock.json
    ├── package.json
    ├── src
    │   ├── Graph.rs
    │   ├── GraphFile
    │   │   ├── io.rs
    │   │   └── properties.rs
    │   ├── GraphFile.rs
    │   ├── decode.rs
    │   ├── io.rs
    │   ├── lib.rs
    │   └── utils.rs
    └── webpack.config.js
```

---

##  Getting Started


### JavsScript & TypeScript

The code is compiled to WebAssembly and bundled into a JavaScript module. You can import the module into your JavaScript or TypeScript project.


The easiest way to get started is by installing the package from npm:

```sh
# Doesnt work yet (package publishing pipeline is not set up yet)
# npm install gt_graph_wasm 
```

To load a graph file, you can use the `from_url` function:

```js
import { Graph } from './gt_graph_wasm';

const graph = await Graph.from_url("<path-to-graph-file>");

```

This runs a fetch request to load the graph file from the given URL.


If you want to load directly from a binary buffer, you can use the `from_data` function:

```js

const graph = await Graph.from_data(<Uint8Array>);

```


### Rust

In theory you can also use the library directly from Rust. However, I'm not too experienced with Rust and I haven't tested this yet. If you want to give me a hand, feel free to open a PR.


## Development

Build with wasm-pack:

```sh
npm run build
```

Host the test page locally:

```sh
npm run serve
```
