use axum::{
    routing::get,
    Router,
    extract::State,
};

use sqlx::SqlitePool;


mod db;

#[tokio::main]
async fn main() {

    //init db
    let pool = db::test_init().await.unwrap();


    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/test_insert", get(test_insert_handler))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn test_insert_handler(State(pool): State<SqlitePool>) -> &'static str {

    db::test_write_to_db(&pool, "Test Name").await.unwrap();
    "Inserted test name into database"
}
