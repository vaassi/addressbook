use std::env;

use sea_orm::{Database, DbBackend, DbConn, DbErr, Statement};

use migration::{ConnectionTrait, Migrator, MigratorTrait};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    // get env vars
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // connection to database
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    // disable case sensetive
    conn.query_one(Statement::from_string(
        DbBackend::Sqlite,
        r#"PRAGMA case_sensitive_like=OFF"#,
    ))
    .await?;

    // apply migrations
    Migrator::up(&conn, None).await?;

    Ok(conn)
}
