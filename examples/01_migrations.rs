use sqlx::postgres::PgPoolOptions;
// use sqlx::mysql::MySqlPoolOptions;
// etc.

#[tokio::main] // Requires the `attributes` feature of `async-std`
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect("postgres://postgres:default@localhost/sqlx_lrn")
    //     .await?;
    let database_url = "postgres://postgres:default@localhost/sqlx_lrn";

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    // ***
    // Test code to delete all migration records from the _sqlx_migrations table.
    // ***
    let delete_result = sqlx::query("DELETE FROM _sqlx_migrations")
        .execute(&db)
        .await
        .unwrap();

    println!("Delete FROM _sqlx_migrations result: {:?}", delete_result);

    let migrate_result = sqlx::migrate!().run(&db).await?;

    println!("Migration result: {:?}", migrate_result);

    Ok(())
}
