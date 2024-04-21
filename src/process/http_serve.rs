use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[derive(Debug, Clone)]
pub struct HttpServeState {
    pub path: PathBuf,
}

/// 处理Http Serve
pub async fn process_http_serve(port: u16, path: PathBuf) -> Result<()> {
    // 监听地址
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Listening on {} and serve path:{:?}", addr, path);

    // 创建路径State
    let state = Arc::new(HttpServeState { path: path.clone() });

    // 创建ServeDir Service
    let serve_dir_service = ServeDir::new(path)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();

    // 创建路由
    let app = Router::new()
        .nest_service("/tower", serve_dir_service)
        .route("/origin/*path", get(handle_dir))
        .with_state(state);

    // 创建TcpListener
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // 监听
    axum::serve::serve(listener, app).await?;

    Ok(())
}

// 处理DIR
async fn handle_dir(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, String> {
    // 完整路径
    let full_path = state.path.join(&path);

    if !full_path.exists() {
        Err("Resource Not Found!".to_owned())
    } else if full_path.is_file() {
        let buf = tokio::fs::read_to_string(&full_path)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Html(buf))
    } else {
        // 读取文件夹内容
        let mut entries = tokio::fs::read_dir(&full_path)
            .await
            .map_err(|e| e.to_string())?;

        let mut list = String::new();

        while let Ok(entry) = entries.next_entry().await {
            if let Some(entry) = entry {
                // 插入buf
                list.push_str(
                    format!(
                        r#"<li><a href="/origin/{}">{}</li>"#,
                        entry
                            .path()
                            .strip_prefix(&state.path)
                            .map_err(|e| e.to_string())?
                            .to_string_lossy(),
                        entry.file_name().to_string_lossy()
                    )
                    .as_str(),
                );
            } else {
                break;
            }
        }

        let template = format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Contents</title>
</head>
<body>
 <ul>
{}</ul>
</body>
</html>
"#,
            list
        );

        Ok(Html(template))
    }
}