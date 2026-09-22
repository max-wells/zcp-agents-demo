use axum::Json;
use axum::extract::State;
use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::domain::order::dto::CreateOrderDto;
use crate::domain::order::error::OrderError;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Order {
    pub unid: Uuid,
    pub sku: String,
    pub quantity: i32,
    pub created_at: OffsetDateTime,
}

impl Order {
    #[tracing::instrument(skip(app_state))]
    pub async fn get_all(
        State(app_state): State<AppState>,
    ) -> Result<Json<Vec<Order>>, OrderError> {
        tracing::info!("fetching all orders");

        let orders = sqlx::query_as!(
            Order,
            "SELECT unid, sku, quantity, created_at FROM orders ORDER BY created_at DESC",
        )
        .fetch_all(&app_state.pool)
        .await?;

        tracing::info!(count = orders.len(), "fetched orders");

        Ok(Json(orders))
    }

    // BUG (intentional, for the demo): quantity is never validated before
    // the INSERT below, so a zero/negative quantity is sent straight to the
    // DB and trips the CHECK (quantity > 0) constraint instead of being
    // rejected here with a clean 4xx. Do not fix — this is the bug the
    // ZCP agents demo is built to surface. OrderError::BadRequest is
    // already wired to a 400 response; the fix is just uncommenting this:
    #[tracing::instrument(skip(app_state))]
    pub async fn create(
        State(app_state): State<AppState>,
        Json(body): Json<CreateOrderDto>,
    ) -> Result<Json<Order>, OrderError> {
        tracing::info!("creating order");

        // if body.quantity <= 0 {
        //     return Err(OrderError::BadRequest("quantity must be greater than 0".into()));
        // }

        let order = sqlx::query_as!(
            Order,
            "INSERT INTO orders (sku, quantity) VALUES ($1, $2) RETURNING unid, sku, quantity, created_at",
            body.sku,
            body.quantity,
        )
        .fetch_one(&app_state.pool)
        .await?;

        tracing::info!(order_id = %order.unid, "order created");

        Ok(Json(order))
    }
}
