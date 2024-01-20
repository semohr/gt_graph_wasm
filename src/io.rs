use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::GraphFile::GraphFile;

/** Create a fetch request to download the binary file and parse it
 * as a graph.
 */
pub async fn fetch_binary(url: String) -> Result<Vec<u8>, JsValue> {
    let mut ops = RequestInit::new();
    ops.method("GET");
    ops.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &ops)?;
    // send headers and request binary
    request.headers();

    let window = web_sys::window().expect("should have a window in this context");
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into()?;

    // get binary data
    let data_ret = JsFuture::from(resp.array_buffer()?).await?;
    // `data` is an `ArrayBuffer`. Convert it to a `Uint8Array`, because that's
    // what we need to work with for now.
    let data = js_sys::Uint8Array::new(&data_ret);
    // Convert this `Uint8Array` into a rust `Vec<u8>`.
    let data = data.to_vec();

    Ok(data)
}
