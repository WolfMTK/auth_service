use std::sync::Arc;
use axum::Router;
use axum::routing::{post, put};
use tokio::net::TcpListener;
use crate::module::Modules;
use crate::routes::user::{create_user, update_user};

pub async fn startup(modules: Arc<Modules>) {
    let user_router = Router::new()
        .route("/",
               post(create_user))
        .route("/:id",
               put(update_user));

    let app = Router::new()
        .nest("/:v/users", user_router)
        .with_state(modules);

    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap_or_else(|_| panic!("TcpListener cannot bind"));

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch"));
}
