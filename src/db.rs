// src/main.rs
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
pub async fn query_db() -> Result<(), sqlx::Error> {
    // 1. Fetch connection string injected by Docker environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:secret@127.0.0.1:3306/rust".to_string());

    // 2. Establish connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Successfully connected to MySQL database!");

    // 3. Query records from the users table
    let users: Vec<User> = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    println!("Retrieved users from database:");
    for user in users {
        println!("- ID: {}, Name: {}", user.id, user.name);
    }

    Ok(())
}
