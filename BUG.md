I try this code.
 
```rust
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Adding a new subscriber", skip(form, pool))]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}
```

```text
#9 227.0 warning: unused import: `chrono::Utc`
#9 227.0  --> src/routes/subscriptions.rs:2:5
#9 227.0   |
#9 227.0 2 | use chrono::Utc;
#9 227.0   |     ^^^^^^^^^^^
#9 227.0   |
#9 227.0   = note: `#[warn(unused_imports)]` on by default
#9 227.0
#9 227.0 warning: unused import: `uuid::Uuid`
#9 227.0  --> src/routes/subscriptions.rs:4:5
#9 227.0   |
#9 227.0 4 | use uuid::Uuid;
#9 227.0   |     ^^^^^^^^^^
```

summary

Uuid와 Utc가 정말 필요없을까.
