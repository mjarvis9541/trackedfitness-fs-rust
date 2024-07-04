#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use trackedfitness::config::get_config;
    use trackedfitness::setup::{
        initialize_db_pool, load_leptos_config, setup_router, start_server,
    };

    tracing_subscriber::fmt().with_thread_ids(true).init();

    let config = get_config();

    let pool = initialize_db_pool(&config.database_url).await;

    let leptos_options = load_leptos_config().await;

    let app = setup_router(pool, leptos_options.clone()).await;

    start_server(&leptos_options.site_addr, app).await;
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
