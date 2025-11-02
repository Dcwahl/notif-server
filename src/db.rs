use sqlx::SqlitePool;


pub async fn test_init() -> Result<SqlitePool, sqlx::Error>{

    //question mark is like a try catch i guess
    let pool = SqlitePool::connect("sqlite://notifications.db?mode=rwc").await?;


    let test_create_table = r#"
        CREATE TABLE IF NOT EXISTS test_table (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
    "#;

    sqlx::query(test_create_table).execute(&pool).await?;

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
