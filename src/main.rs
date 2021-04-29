use std::time::Duration;
use sqlx::{postgres::PgPoolOptions, types::{chrono::{NaiveDateTime, Utc}}};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .connect("postgres://root@localhost:26257/sensors")
        .await
        .unwrap();
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS public.boxes (
            id UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
            name STRING,
            time_last_received TIMESTAMP,
            imei STRING,
            phone_number STRING
        );"
    )
    .execute(&pool)
    .await?;

    let now = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
    let row = sqlx::query!("insert into boxes (name, time_last_received, imei, phone_number) values ($1, $2, $3, $4) returning id",
        "aaa", now, "0000", "jozo"
    )
    .fetch_one(&pool)
    .await?;
    println!("inserted id: {:?}", row);
    Ok(())
}
