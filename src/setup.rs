use std::net::SocketAddr;

use leptos::{get_configuration, use_context, *};
use leptos_axum::{generate_route_list, LeptosRoutes};

use axum::response::IntoResponse;
use axum::Router;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::Level;

use crate::app::App;
use crate::error::{Error, Result};
use crate::middleware::auth_token_middleware;

pub fn get_pool() -> Result<PgPool> {
    use_context::<PgPool>().ok_or(Error::InternalServer)
}

pub async fn initialize_db_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("could not create a database pool")
}

pub async fn load_leptos_config() -> LeptosOptions {
    get_configuration(None).await.unwrap().leptos_options
}

pub async fn setup_router(pool: PgPool, leptos_options: LeptosOptions) -> Router {
    let routes = generate_route_list(App);
    let site_root = &leptos_options.site_root;

    let favicon = ServeFile::new(format!("{}/favicon.ico", site_root));
    let image_dir = ServeDir::new(format!("{}/images", site_root));
    let site_pkg_dir = ServeDir::new(format!("{}/{}", site_root, leptos_options.site_pkg_dir));

    let shared_leptos_options = leptos_options.clone();

    let context_provider = {
        let pool = pool.clone();
        move || {
            provide_context(pool.clone());
        }
    };

    Router::new()
        .route_service("/favicon.ico", favicon)
        .nest_service("/images", image_dir)
        .nest_service("/pkg", site_pkg_dir)
        .leptos_routes_with_context(
            &shared_leptos_options,
            routes,
            context_provider.clone(),
            App,
        )
        .fallback(|req| async move {
            let handler = leptos_axum::render_app_to_stream_with_context(
                shared_leptos_options,
                context_provider,
                App,
            );
            handler(req).await.into_response()
        })
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn(auth_token_middleware))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO))
                        .on_failure(DefaultOnFailure::new().level(Level::ERROR)),
                ),
        )
        .with_state(leptos_options)
}

pub async fn start_server(addr: &SocketAddr, app: Router) {
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
