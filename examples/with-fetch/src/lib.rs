use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn fetch_data(url: String) -> Result<String, JsValue> {
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;

    Ok(body)
}
