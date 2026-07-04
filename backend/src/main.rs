use axum::{
    routing::{get, post},
    Router,
};
mod db;
mod handlers;
mod models;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::db::get_db_pool().await?;
    eprintln!("[INFO] Connected to database");
    let app = Router::new()
        .route("/get", get(handlers::project_handler::get_all_projects))
        .route("/create", post(handlers::project_handler::insert_project))
        .route(
            "/delete/{id}",
            post(handlers::project_handler::delete_project),
        )
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
