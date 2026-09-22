use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateOrderDto {
    pub sku: String,
    pub quantity: i32,
}
