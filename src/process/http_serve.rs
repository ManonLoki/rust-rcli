use std::{path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Serialize;
use tera::{Context, Tera};
use tower_http::services::ServeDir;

/// Http静态文件服务状态
#[derive(Debug, Clone)]
pub struct HttpServeState {
    pub path: PathBuf,
}

/// 模版数据列表项
#[derive(Serialize)]
pub struct TemplateDataItem {
    /// 链接
    pub link: String,
    /// 文本
    pub text: String,
}

/// 模版数据
#[derive(Serialize)]
pub struct TemplateData {
    /// 数据列表
    pub list: Vec<TemplateDataItem>,
}

/// 处理Http Serve
pub async fn process_http_serve(port: u16, path: PathBuf) -> anyhow::Result<()> {
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
        .route("/origin", get(handle_root_path))
        .route("/origin/*path", get(handle_combine_path))
        .with_state(state);

    // 创建TcpListener
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // 监听
    axum::serve::serve(listener, app).await?;

    Ok(())
}

/// 处理 /origin
async fn handle_root_path(
    state: State<Arc<HttpServeState>>,
) -> Result<impl IntoResponse, StaticFileError> {
    handle_path(&state.path, None).await
}
/// 处理 /origin/*
async fn handle_combine_path(
    state: State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> Result<impl IntoResponse, StaticFileError> {
    handle_path(&state.path, Some(path)).await
}

// 处理DIR
async fn handle_path(
    root_path: &PathBuf,
    combine_path: Option<String>,
) -> Result<impl IntoResponse, StaticFileError> {
    // 完整路径
    let full_path = if let Some(path) = combine_path {
        root_path.join(path)
    } else {
        root_path.clone()
    };

    // 判断路径是否存在
    if !full_path.exists() {
        Err(StaticFileError::NotFound)
    } else if full_path.is_file() {
        // 读取文件内容
        let buf = tokio::fs::read_to_string(&full_path)
            .await
            .map_err(|e| StaticFileError::IoError(format!("Read File Error:{}", e)))?;

        Ok(buf.into_response())
    } else {
        // 读取文件夹内容
        let mut entries = tokio::fs::read_dir(&full_path)
            .await
            .map_err(|e| StaticFileError::IoError(format!("Read Dir Entries Error:{}", e)))?;

        // 创建模版
        let template = Tera::new("templates/**/*")
            .map_err(|e| StaticFileError::IoError(format!("模版初始化错误:{}", e)))?;

        // 模版数据
        let mut template_data = TemplateData { list: vec![] };

        // 遍历并生成
        while let Ok(entry) = entries.next_entry().await {
            if let Some(entry) = entry {
                // 获取链接
                let link = entry
                    .path()
                    .strip_prefix(root_path)
                    .map_err(|e| {
                        StaticFileError::IoError(format!("Get Entry Reative Path Error:{}", e))
                    })?
                    .to_string_lossy()
                    .to_string();
                // 获取文本呢
                let text = format!(
                    "{}{}",
                    entry.file_name().to_string_lossy(),
                    if entry.path().is_file() { "" } else { "/" }
                );

                // 加入模版中
                template_data.list.push(TemplateDataItem { link, text });
            } else {
                break;
            }
        }

        // 渲染HTML
        let html = template
            .render(
                "index.html",
                &Context::from_serialize(&template_data)
                    .map_err(|e| StaticFileError::IoError(format!("模版数据错误:{}", e)))?,
            )
            .map_err(|e| StaticFileError::IoError(format!("模版渲染错误:{}", e)))?;

        Ok(Html(html).into_response())
    }
}

/// 通过实现IntoResponse Trait 实现自定义异常
pub enum StaticFileError {
    NotFound,
    IoError(String),
}

impl IntoResponse for StaticFileError {
    fn into_response(self) -> axum::response::Response {
        match self {
            StaticFileError::NotFound => {
                (StatusCode::NOT_FOUND, "Resource Not Found!").into_response()
            }
            StaticFileError::IoError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
        }
    }
}
