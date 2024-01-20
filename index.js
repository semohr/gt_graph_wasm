import { Graph } from './pkg';

// Attach the module to the window object.
const g = await Graph.from_url("/test_data/network.gt.zst");

console.log(g);