use std::env;

use sea_orm::{Database, DbConn, DbErr};

use migration::{Migrator, MigratorTrait};

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    // get env vars
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // connection to database
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    // apply migrations
    Migrator::up(&conn, None).await?;

    Ok(conn)
}
