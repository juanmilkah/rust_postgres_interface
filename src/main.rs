use dotenv::dotenv;
use rust_postgres_interface::{create_user, delete_user, get_user, update_user_email};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL string must be set in env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect database");

    println!("Connected to the database");

    //create user
    create_user(&pool, "Jane Doe", "janedoe@gmail.com")
        .await
        .expect("Failed to create a new user");

    //get user
    let user = get_user(&pool, "janedoe@gmail.com")
        .await
        .expect("Failed to get user by email");
    println!("{:?}", user);

    //update user
    update_user_email(&pool, 1, "johndoe@gmail.com")
        .await
        .expect("Failed to update user email");

    //delete user
    delete_user(&pool, "johndoe@gmail.com")
        .await
        .expect("Failed to delete user by email");

    println!("Operation completed");
}
