use axum::{
    Router,
    extract::{Query, State},
    routing::{get, post},
    Json,
    http::StatusCode
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::time::SystemTime;

mod db;

static START_TIME: std::sync::LazyLock<SystemTime> = std::sync::LazyLock::new(SystemTime::now);

#[tokio::main]
async fn main() {
    //init db
    let pool = db::init_db().await.unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check))
        .route("/test_insert", get(test_insert_handler))
        .route("/notifications", get(fetch_notifications).post(create_notification))
        .route("/debug/notifications", get(debug_all_notifications))
        .with_state(pool);

    // run our app with hyper, listening globally on port 8081
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    uptime_seconds: u64,
    database: String,
    version: String,
}

async fn health_check(State(pool): State<SqlitePool>) -> (StatusCode, Json<HealthResponse>) {
    // Check DB connection
    let db_status = match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => "connected",
        Err(_) => "error",
    };

    // Calculate uptime
    let uptime = START_TIME.elapsed().unwrap_or_default().as_secs();

    let response = HealthResponse {
        status: if db_status == "connected" { "healthy".to_string() } else { "unhealthy".to_string() },
        uptime_seconds: uptime,
        database: db_status.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    let status_code = if db_status == "connected" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(response))
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
    scheduled_for: Option<String>,
}

#[derive(Serialize)]
struct NotificationDebug {
    id: i64,
    title: String,
    message: String,
    timestamp: String,
    scheduled_for: Option<String>,
}

async fn create_notification(pool: State<SqlitePool>, notification: Json<CreateNotificationRequest>) -> Json<Notification> {
    println!("Creating notification: {:?}", notification.title);
    let title = &notification.title;
    let message = &notification.message;
    let scheduled_for = &notification.scheduled_for;
    let row = db::add_notification(&pool, title, message, scheduled_for.as_deref()).await.unwrap();

    let (id, title, message, timestamp) = row;
    println!("Created notification with ID: {}", id);
    Json(Notification { id, title, message, timestamp })
}

async fn fetch_notifications(
    State(pool): State<SqlitePool>,
    Query(params): Query<NotificationQuery>,
) -> Json<Vec<Notification>> {
    //println!("Fetching notifications");
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

async fn debug_all_notifications(State(pool): State<SqlitePool>) -> Json<Vec<NotificationDebug>> {
    let rows = db::get_all_notifications(&pool)
        .await
        .unwrap_or_else(|e| {
            eprintln!("DB error: {}", e);
            vec![]
        });

    let notifications: Vec<NotificationDebug> = rows.into_iter()
        .map(|(id, title, message, timestamp, scheduled_for)| NotificationDebug {
            id, title, message, timestamp, scheduled_for
        })
        .collect();

    Json(notifications)
}
