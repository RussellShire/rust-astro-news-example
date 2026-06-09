use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ExternalApiNewsItem {
    title: String,
    description: Option<String>,
    url: String,
}

#[derive(Deserialize)]
struct ExternalApiResponse {
    data: Vec<ExternalApiNewsItem>,
}

#[derive(Serialize)]
struct ManagedArticle {
    title: String,
    snippet: String,
    url: String,
}

// 1. Removed wasm_bindgen macro
// 2. Used standard C-FFI export so Wasmer can bind to it natively
#[no_mangle]
pub extern "C" fn compile_skeleton() {
    // In a production environment, you would use unsafe {} memory
    // pointers here to read the JSON string passed from the native host.
    // For this architectural demo, we just expose the function so
    // Wasmer can successfully instantiate the module buffer.
}
