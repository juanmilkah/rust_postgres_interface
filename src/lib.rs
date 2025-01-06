#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub async fn create_user(pool: &sqlx::PgPool, name: &str, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("insert into users (name, email) values ($1, $2)")
        .bind(name)
        .bind(email)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_user(pool: &sqlx::PgPool, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("select id, name, email from users where email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn update_user_email(
    pool: &sqlx::PgPool,
    user_id: i32,
    new_email: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("update users set email = $1 where id = $2")
        .bind(new_email)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_user(pool: &sqlx::PgPool, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("delete from users where email = $1")
        .bind(email)
        .execute(pool)
        .await?;

    Ok(())
}
