use chrono::{DateTime, Utc};
#[derive(Clone)]
pub struct Invoice {
    pub invoice_id: u32,
    pub customer_id: u64,
    pub subtotal: f32,
    pub tax: f32,
    pub total: f32,
    pub currency: String,
    pub datetime: DateTime<Utc>,
    pub balance: f32,
    pub comments: String,
}

pub struct FraudResult {
    pub company_id: u64,
    pub datetime: DateTime<Utc>,
    pub fraud_suspected: bool,
}
