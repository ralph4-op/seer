// src-tauri/src/lib.rs
#[tauri::command]
async fn add_post(content: String) -> Result<String, String> {
    // Call JavaScript to add a post to OrbitDB
    let result = tauri::api::shell::execute("node", &["src/scripts/main.js", "add_post", &content])
        .map_err(|e| e.to_string())?;
    Ok(result.stdout)
}