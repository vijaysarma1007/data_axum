use sea_orm::{Database};
mod routes;


pub async fn run(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app =  routes::create_routes(database);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}