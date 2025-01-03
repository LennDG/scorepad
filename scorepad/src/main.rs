#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::response::{IntoResponse, Response};
    use axum::routing::get;
    use axum::{middleware, Router};
    use http::StatusCode;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use scorepad::app::*;
    use scorepad::file_server::{cache_control, file_and_error_handler};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    async fn health_check_handler() -> Response {
        StatusCode::OK.into_response()
    }

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .route("/health", get(health_check_handler))
        .fallback(file_and_error_handler)
        //.layer(middleware::map_response(cache_control))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
