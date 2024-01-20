use gt_graph_wasm::graph::Graph;

use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn test_from_url() {
    let url = "https://networks.skewed.de/net/advogato/files/advogato.gt.zst".to_string();

    // Create a promise that is ready on the next tick of the micro task queue.
    let graph = Graph::from_url(url).await.unwrap();
}
