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
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let users: Vec<User> = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    println!("\nUsers:\n");
    for user in users {
        println!("- ID: {}, Name: {}", user.id, user.name);
    }

    Ok(())
}
