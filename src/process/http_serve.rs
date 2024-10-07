use anyhow::Result;
use axum::{
    // extract::{Path, State},
    // http::StatusCode,
    // routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    info!("process_http_serve: path={:?}, port={}", path, port);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(&state.path)
        .append_index_html_on_directories(true)
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_gzip()
        .precompressed_zstd();
    let router = Router::new()
        // .route("/*path", get(file_handler))
        .nest_service("/", dir_service)
        .with_state(Arc::new(state));
    info!("listening on {:?}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

// async fn file_handler(
//     State(state): State<Arc<HttpServeState>>,
//     Path(path): Path<String>,
// ) -> (StatusCode, String) {
//     let p = std::path::Path::new(&state.path).join(path);
//     info!("file_handler: path={:?}", p);
//     if !p.exists() {
//         warn!("file not found: path={:?}", p);
//         (
//             StatusCode::NOT_FOUND,
//             format!("file:{} not found", p.display()),
//         )
//     } else {
//         match tokio::fs::read_to_string(p).await {
//             Ok(content) => (StatusCode::OK, content),
//             Err(err) => {
//                 warn!("file not found, err={:?}", err);
//                 (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
//             }
//         }
//     }
// }
