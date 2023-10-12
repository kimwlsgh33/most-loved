use axum::{extract::State, http, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> Result<(http::StatusCode, Json<Quote>), http::StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book, quote, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&quote.id)
    .bind(&quote.book)
    .bind(&quote.quote)
    .bind(&quote.created_at)
    .bind(&quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, Json(quote))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn read_quotes(State(pool): State<PgPool>) -> Result<Json<Vec<Quote>>, http::StatusCode> {
    let res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(qoutes) => Ok(Json(qoutes)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
