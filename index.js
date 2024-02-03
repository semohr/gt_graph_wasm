import { Graph } from './pkg';

// Attach the module to the window object.
const g = await Graph.from_url("/test_data/network.gt.zst");

console.log(g);

console.log("vertices", g.num_vertices);
console.log("edges", g.num_edges);

console.log("vertex", g.vertices());
console.log("edge", g.edges());

