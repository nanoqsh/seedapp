use {
    axum::{
        http::{Request, StatusCode},
        response::{IntoResponse, Response},
        routing::get_service,
        Router, Server,
    },
    std::{io, net::SocketAddr},
};

fn main() {
    use tokio::runtime::Builder;

    init_tracing();

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime")
        .block_on(serve());
}

fn init_tracing() {
    use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    const APP_NAME: &str = env!("CARGO_PKG_NAME");

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{APP_NAME}=debug,tokio=debug,tower_http=debug").into()
            }),
        )
        .with(fmt::layer())
        .init();
}

async fn serve() {
    use {tower::service_fn, tower_http::services::ServeDir};

    let service = ServeDir::new("static").not_found_service(service_fn(handle_not_found));

    let app =
        Router::new().fallback_service(get_service(service).handle_error(|_| async {
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..")
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("serve at http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("serve");
}

async fn handle_not_found<T>(_: Request<T>) -> Result<Response, io::Error> {
    Ok((StatusCode::NOT_FOUND, "File not found").into_response())
}
