use axum::{
    Router,
    extract::{Query, State},
    routing::{get, post},
    Json
};
use serde::{Deserialize, Serialize};
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
        .route("/notifications", get(fetch_notifications).post(create_notification))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn test_insert_handler(State(pool): State<SqlitePool>) -> &'static str {
    db::test_write_to_db(&pool, "Test Name").await.unwrap();
    "Inserted test name into database"
}

#[derive(Deserialize)]
struct NotificationQuery {
    id: Option<i64>, // Option because it might not be provided
}

//the #derive thing lets us convert to json easily
#[derive(Serialize)]
struct Notification {
    id: i64,
    title: String,
    message: String,
    timestamp: String,
}

#[derive(Deserialize)]
struct CreateNotificationRequest {
    title: String,
    message: String,
}

async fn create_notification(pool: State<SqlitePool>, notification: Json<CreateNotificationRequest>) -> Json<Notification> {
    let title = &notification.title;
    let message = &notification.message;
    let row = db::add_notification(&pool, title, message).await.unwrap();

    let (id, title, message, timestamp) = row;

    Json(Notification { id, title, message, timestamp })
}

async fn fetch_notifications(
    State(pool): State<SqlitePool>,
    Query(params): Query<NotificationQuery>,
) -> Json<Vec<Notification>> {
    let last_seen_id = params.id.unwrap_or(-1);

    let rows = db::get_latest_notifications(&pool, &last_seen_id)
        .await
        .unwrap_or_else(|e| {
            eprintln!("DB error: {}", e);
            vec![]  // return empty vec on error
        });
    
    let notifications: Vec<Notification> = rows.into_iter()
        .map(|(id, title, message, timestamp)| Notification { id, title, message, timestamp })
        .collect();

    Json(notifications)
}
