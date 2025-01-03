use axum::extract::State;
use axum::response::Response as AxumResponse;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use http::{HeaderValue, Uri};
use leptos::config::LeptosOptions;
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(
    State(options): State<LeptosOptions>,
    req: Request<Body>,
) -> AxumResponse {
    let root = &options.site_root;
    let (parts, _) = req.into_parts();

    let mut static_parts = parts.clone();
    static_parts.headers.clear();
    if let Some(encodings) = parts.headers.get("accept-encoding") {
        static_parts
            .headers
            .insert("accept-encoding", encodings.clone());
    }

    let res = get_static_file(Request::from_parts(static_parts, Body::empty()), root)
        .await
        .unwrap();

    res.into_response()
}

async fn get_static_file(
    request: Request<Body>,
    root: &str,
) -> Result<Response<Body>, (StatusCode, String)> {
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    match ServeDir::new(root)
        .precompressed_gzip()
        .precompressed_br()
        .oneshot(request)
        .await
    {
        Ok(res) => Ok(res.into_response()),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error serving files: {err}"),
        )),
    }
}

pub async fn cache_control(uri: Uri, mut res: Response<Body>) -> Response<Body> {
    // Get the path from the request
    let path = uri.path();

    if path.ends_with(".wasm") || path.ends_with(".js") || path.ends_with(".css") {
        res.headers_mut().insert(
            "Cache-Control",
            HeaderValue::from_static("public, max-age=31536000, immutable"),
        );
    } else if let Some(content_type) = res.headers().get("content-type") {
        // For HTML responses, ensure no caching
        if content_type.to_str().unwrap_or("").contains("text/html") {
            res.headers_mut().insert(
                "Cache-Control",
                HeaderValue::from_static("no-cache, must-revalidate"),
            );
        }
    }

    res
}
