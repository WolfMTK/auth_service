use std::env;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;

use crate::module::Modules;
use crate::routes::user::{create_user, get_user, update_user};

pub async fn startup(modules: Arc<Modules>) {
    let user_router = Router::new()
        .route("/", post(create_user))
        .route("/:id", get(get_user).put(update_user));

    let app = Router::new()
        .nest("/:v/users", user_router)
        .with_state(modules);

    let addr = SocketAddr::from(init_addr());
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("TcpListener cannot bind"));
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch"));
}

fn init_addr() -> (IpAddr, u16) {
    let ip_addr = match env::var("HOST") {
        Ok(val) => val.parse::<IpAddr>(),
        Err(_) => "127.0.0.1".parse::<IpAddr>(),
    }.expect("HOST is invalid.");

    let port = match env::var("PORT") {
        Ok(val) => val.parse::<u16>(),
        Err(_) => Ok(8000)
    }.expect("PORT is invalid.");

    (ip_addr, port)
}
