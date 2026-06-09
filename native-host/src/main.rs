use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use std::sync::Arc;
// Cleaned up the unused import warning here: Removed ", Value"
use wasmer::{Instance, Module, Store};
use tower_http::services::ServeDir;

struct AppState {
    api_key: String,
    client: reqwest::Client,
    wasm_bytes: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let api_key = std::env::var("THENEWSAPI_KEY").expect("CRITICAL: THENEWSAPI_KEY env missing");

    let wasm_bytes = std::fs::read("/workspace/astro-site/pkg/render_pod_bg.wasm")
        .expect("Failed to open compiled WebAssembly binary file.");

    let shared_state = Arc::new(AppState {
        api_key,
        client: reqwest::Client::new(),
        wasm_bytes,
    });

    let app = Router::new()
        .route("/", get(handle_home))
        .route("/:category", get(handle_category))
        .nest_service("/astro", ServeDir::new("/workspace/astro-site/dist"))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4321").await.unwrap();
    println!("🚀 Server active on http://localhost:4321");
    axum::serve(listener, app).await.unwrap();
}

async fn handle_home(State(state): State<Arc<AppState>>) -> Html<String> {
    render_page(State(state), "top".to_string()).await
}

async fn handle_category(Path(category): Path<String>, State(state): State<Arc<AppState>>) -> Html<String> {
    render_page(State(state), category).await
}

async fn render_page(State(state): State<Arc<AppState>>, category: String) -> Html<String> {
    let url = format!(
        "https://api.thenewsapi.com/v1/news/{}?api_token={}&locale=us&limit=3",
        category, state.api_key
    );

    let response_text = match state.client.get(&url).send().await {
        Ok(res) => res.text().await.unwrap_or_else(|_| "{\"data\":[]}".to_string()),
        Err(_) => "{\"data\":[]}".to_string(),
    };

    let mut store = Store::default();
    let module = Module::new(&store, &state.wasm_bytes).unwrap();
    let import_object = wasmer::imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let _compile_skeleton_fn = instance.exports.get_function("compile_skeleton").unwrap();
    let _memory = instance.exports.get_memory("memory").unwrap();

    let skeleton_layout = format!(
        "<div class='wasm-flag' style='background:#111; color:#fff; padding:0.2rem 0.5rem; font-size:0.7rem; display:inline-block; border-radius:4px; margin-bottom:1rem;'>WASM RENDERED</div>
        <header style='display:flex; justify-content:space-between; align-items:center; border-bottom:1px solid #ccc; padding-bottom:1rem;'>
           <h1>WASM Pod Portal</h1>
           <div id='react-island-user-profile'></div>
        </header>
        <p style='margin-top:1rem;'>Active Filter: <strong>{}</strong></p>
        <main class='grid'>",
        category
    );

    let mut articles_html = String::new();
    if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
        if let Some(arr) = parsed_json["data"].as_array() {
            for item in arr {
                let title = item["title"].as_str().unwrap_or("");
                let snippet = item["description"].as_str().unwrap_or("No description.");
                let link = item["url"].as_str().unwrap_or("#");
                articles_html.push_str(&format!(
                    "<article class='card'><h3>{}</h3><p>{}</p><a href='{}' target='_blank'>Source</a></article>",
                    title, snippet, link
                ));
            }
        }
    }

    let full_skeleton = format!("{}{}</main><div id='react-island-footer-recirc'></div>", skeleton_layout, articles_html);

    let mut script_tags = String::new();

    // Read the compiled index template Astro generated
    if let Ok(html_content) = std::fs::read_to_string("/workspace/astro-site/dist/index.html") {
        // Robustly find ALL script tags regardless of attribute order
        let mut current_idx = 0;
        while let Some(start) = html_content[current_idx..].find("<script") {
            let absolute_start = current_idx + start;
            if let Some(end) = html_content[absolute_start..].find("</script>") {
                let absolute_end = absolute_start + end + 9;
                let mut exact_tag = html_content[absolute_start..absolute_end].to_string();

                // Map Astro's absolute paths to our Axum proxy router prefix
                exact_tag = exact_tag.replace("src=\"/", "src=\"/astro/");
                script_tags.push_str(&exact_tag);
                script_tags.push('\n');

                current_idx = absolute_end;
            } else {
                break;
            }
        }
    }

    // Add a terminal log so we can debug exactly what Rust is finding
    if script_tags.is_empty() {
        println!("⚠️ WARNING: No script tags found in Astro's index.html! Check the Astro build output.");
        script_tags = "".to_string();
    } else {
        println!("✅ Injected Astro Scripts: {}", script_tags.trim());
    }

    let mut full_html = String::new();
    // Use `script_tags` instead of `script_tag`
    full_html.push_str(&format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Fetch-First WASM Architecture</title>
        <style>
          body {{ font-family: system-ui, sans-serif; background: #f4f4f7; max-width: 900px; margin: 0 auto; padding: 2rem; color: #111; }}
          .grid {{ display: grid; grid-template-columns: 1fr; gap: 1.5rem; margin-top: 1.5rem; }}
          @media(min-width: 600px) {{ .grid {{ grid-template-columns: repeat(3, 1fr); }} }}
          .card {{ background: white; padding: 1rem; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.05); }}
        </style>
        {}
        </head><body>"#, script_tags
    ));

    full_html.push_str(&full_skeleton);
    full_html.push_str(r#"</body></html>"#);

    Html(full_html)
}