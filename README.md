<p align="center">
    <h1 align="center">GT_GRAPH_WASM</h1>
</p>
<p align="center">
    <em>JS & Rust reader for the gt file format</em>
</p>
<p align="center">
        <img src="https://github.com/semohr/gt_graph_wasm/actions/workflows/rust.yml/badge.svg?branch=main" alt="test_status">
	<img src="https://img.shields.io/github/last-commit/semohr/gt_graph_wasm?style=flat&color=0080ff" alt="last-commit">
	<img src="https://img.shields.io/github/license/semohr/gt_graph_wasm?style=flat&color=0080ff" alt="license">
	<img src="https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=Rust&logoColor=white" alt="Rust">
	<img src="https://img.shields.io/badge/Webpack-8DD6F9.svg?style=flat&logo=Webpack&logoColor=black" alt="Webpack">
<p>

<hr>

---
## About

This is a WebAssembly module for loading graphs in the [gt file format](https://graph-tool.skewed.de/static/doc/gt_format.html). It is written in Rust and compiled to WebAssembly using [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/). The module is then bundled into a JavaScript module such that it can be used in JavaScript and TypeScript projects.

At the moment only loading graphs is supported. Writing capabilities might be added in the future but are not a priority for me as this library was developed to allow visualizing gt graphs in the browser.


##  Getting Started


### JavsScript & TypeScript

The code is compiled to WebAssembly and bundled into a JavaScript module. You can import the module into your JavaScript or TypeScript project.


The easiest way to get started is by installing the package from npm:

```sh
npm i @semohr/gt_graph_wasm
```


#### Loading a graph

To load a graph file, you can use the `from_url`, `from_data` or `from_netzschleuder` functions:

```js
import { Graph } from "@semohr/gt_graph_wasm";

//This runs a fetch request to load the graph file from the given URL.
const graph = await Graph.from_url("<path-to-graph-file>");

//You can also load a graph directly from a binary buffer
const graph = await Graph.from_data(<Uint8Array>);

//You can also load a graph directly from the Netzschleuder Repository
const graph = await Graph.from_netzschleuder("advogato");

// for databases with multiple graphs, you can specify the graph name
const graph = await Graph.from_netzschleuder("fresh_webs", "AkatoreA");

console.log(graph);
```


#### Accessing properties

You can access the graph properties using the `graph_properties`, `vertex_properties` and `edge_properties` methods. If you want to find a list of all available properties, you can use the `graph_property_names`, `vertex_property_names` and `edge_property_names` methods.

```js
const graph = await Graph.from_url("<path-to-graph-file>");

// get a list of all available graph properties
console.log(graph.graph_property_names());

// get the value of a graph property
console.log(graph.graph_properties("<name>"));
```

Generally the property methods return a typed array. Depending on the property type it will be cast to the appropriate JavaScript type. For example, a `Vec<f32>` property will be cast to a `Float32Array`.

#### Accessing the graph structure

You can access the graph structure using the `vertices` and `edges` getter. These methods return a typed array of the vertex and edge indices.

```js

// Defined as getters
const num_vertices = graph.num_vertices;
const num_edges = graph.num_edges;

 // returns a BigUint64Array
const vertices = graph.vertices()

// returns a BigUint64Array, BigUint64Array (from_vertex, to_vertex)
const edges = graph.edges();

```

You can also get the `in_neighbors` and `out_neighbors` of a given vertex.

```js
const vertex = 0;
const in_edges = graph.in_neighbors(vertex);
const out_edges = graph.out_neighbors(vertex);
```


### Limitations

JavaScript (nor rust) do support 128-bit integers and 128-bit floating point numbers out of the box. Therefore, any 128 bit value is rounded, here a `BigInt64Array`or `Float64Array` is returned to JS. There is a loss of precision when using 128-bit floats. This is not a problem for most use cases, but it is something to be aware of.

Further this limits the maximum number of vertices and edges to 2^64. This should be sufficient for most use cases, but it is something to be aware of.

### Rust

In theory you can also use the library directly in Rust. However, I'm not too experienced with Rust and I haven't tested this yet. Generally one would have to introduce a feature flag and move all wasm specific code into this feature. Then one should be able to use the library as a normal Rust library.

If you want to give me a hand, feel free to open a PR. 


## Development

Build with wasm-pack:

```sh
npm run build
```

Host the test page locally:

```sh
npm run serve
```
