use axum::http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    };
use dotenv::dotenv;
use tower_http::cors::CorsLayer;
use clean_architecture::{api::router::create_router, infrastructure::surreal::surreal_context::connect_db};

#[tokio::main]
async fn main() {
    // load environments from .env
    dotenv().ok();

    connect_db().await.unwrap();
    
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
