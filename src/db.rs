use sqlx::SqlitePool;


pub async fn test_init() -> Result<SqlitePool, sqlx::Error>{

    //question mark is like a try catch i guess
    let pool = SqlitePool::connect("sqlite://notifications.db?mode=rwc").await?;


    // let test_create_table = r#"
    //     CREATE TABLE IF NOT EXISTS test_table (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         name TEXT NOT NULL
    //     );
    // "#;

    // sqlx::query(test_create_table).execute(&pool).await?;


    let create_notifications_table = r#"
        CREATE TABLE IF NOT EXISTS notifications (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            message TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    "#;

    sqlx::query(create_notifications_table).execute(&pool).await?;


    Ok(pool)
}

pub async fn test_write_to_db(pool: &SqlitePool, name: &str) -> Result<(), sqlx::Error> {
    let insert_query = r#"
        INSERT INTO test_table (name) VALUES (?1);
    "#;

    sqlx::query(insert_query)
        .bind(name)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn add_notification(pool: &SqlitePool, title: &str, message: &str) -> Result<(i64, String, String, String), sqlx::Error> {
    let insert_query = r#"
        INSERT INTO notifications (title, message)
        VALUES (?1, ?2)
        RETURNING id, title, message, timestamp;
    "#;

    let row = sqlx::query_as::<_, (i64, String, String, String)>(insert_query)
        .bind(title)
        .bind(message)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub async fn get_latest_notifications(pool: &SqlitePool, id: &i64) -> Result<Vec<(i64, String, String, String)>, sqlx::Error> {
    let select_query = r#"
        SELECT id, title, message, timestamp FROM notifications WHERE id > ?1 ORDER BY id ASC;
    "#;

    let rows = sqlx::query_as::<_, (i64, String, String, String)>(select_query)
        .bind(id)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}
