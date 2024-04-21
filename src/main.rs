mod handlers;
mod routers;
mod user;
#[tokio::main]
async fn main() {
    let app = routers::app();

    let listenr = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    axum::serve(listenr, app).await.unwrap();
}
