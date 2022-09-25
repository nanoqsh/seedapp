use {
    axum::{
        http::{Request, StatusCode},
        response::{IntoResponse, Response},
        routing::get_service,
        Router, Server,
    },
    std::{io, net::SocketAddr},
    tokio::runtime::Builder,
    tower::service_fn,
    tower_http::services::ServeDir,
};

fn main() {
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("build tokio runtime")
        .block_on(serve())
}

async fn serve() {
    let service = ServeDir::new("static").not_found_service(service_fn(handle_not_found));

    let app =
        Router::new().fallback(get_service(service).handle_error(|_| async {
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..")
        }));

    let addr = SocketAddr::from(([0; 4], 3000));
    println!("serve at http://{addr}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("serve");
}

async fn handle_not_found<T>(_: Request<T>) -> Result<Response, io::Error> {
    Ok((StatusCode::NOT_FOUND, "File not found").into_response())
}
