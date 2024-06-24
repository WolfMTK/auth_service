use axum::routing::get;
use axum::Router;

struct Address {
    host: String,
    port: String,
}

fn get_addr() -> Address {
    let port: String;
    let host: String;

    match std::env::var("SERVER_PORT") {
        Ok(val) => port = val,
        Err(_) => port = String::from("8000"),
    }

    match std::env::var("SERVER_HOST") {
        Ok(val) => host = val,
        Err(_) => host = String::from("0.0.0.0"),
    }
    Address { host, port }
}

async fn hello() -> String {
    String::from("Hello World")
}

pub async fn start_server() {
    let addr = get_addr();
    let app = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", addr.host, addr.port))
        .await
        .unwrap();

    println!(
        "Server has launched from http://{}:{}",
        addr.host, addr.port
    );
    axum::serve(listener, app).await.unwrap();
}
