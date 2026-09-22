use axum::Router;

use crate::app_state::AppState;
use crate::domain::order::order::Order;

pub fn orders_routes() -> Router<AppState> {
    Router::new().route(
        "/orders",
        axum::routing::get(Order::get_all).post(Order::create),
    )
}
