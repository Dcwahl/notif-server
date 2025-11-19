use sqlx::SqlitePool;


pub async fn init_db() -> Result<SqlitePool, sqlx::Error>{

    //question mark is like a try catch i guess
    let pool = SqlitePool::connect("sqlite://notifications.db?mode=rwc").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
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

pub async fn add_notification(pool: &SqlitePool, title: &str, message: &str, scheduled_for: Option<&str>) -> Result<(i64, String, String, String), sqlx::Error> {
    let insert_query = r#"
        INSERT INTO notifications (title, message, scheduled_for)
        VALUES (?1, ?2, ?3)
        RETURNING id, title, message, timestamp;
    "#;

    let row = sqlx::query_as::<_, (i64, String, String, String)>(insert_query)
        .bind(title)
        .bind(message)
        .bind(scheduled_for)
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub async fn get_latest_notifications(pool: &SqlitePool, id: &i64) -> Result<Vec<(i64, String, String, String)>, sqlx::Error> {
    let select_query = r#"
        SELECT id, title, message, timestamp FROM notifications
        WHERE id > ?1
        AND (scheduled_for IS NULL or datetime(scheduled_for) <= datetime('now'))
        ORDER BY id ASC;
    "#;
    let rows = sqlx::query_as::<_, (i64, String, String, String)>(select_query)
        .bind(id)
        .fetch_all(pool)
        .await?;

    //println!("Fetched {} notifications from DB", rows.len());

    Ok(rows)
}

pub async fn get_all_notifications(pool: &SqlitePool) -> Result<Vec<(i64, String, String, String, Option<String>)>, sqlx::Error> {
    let select_query = r#"
        SELECT id, title, message, timestamp, scheduled_for FROM notifications ORDER BY id ASC;
    "#;

    let rows = sqlx::query_as::<_, (i64, String, String, String, Option<String>)>(select_query)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}
