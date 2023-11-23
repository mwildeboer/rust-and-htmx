use crate::environment::Environment;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod root;

pub async fn serve(env: &Environment) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "htmx=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], env.port));
    let app = router().layer(TraceLayer::new_for_http());

    info!("Running on http://localhost:{}", env.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new().merge(root::router())
}
