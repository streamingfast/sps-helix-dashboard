use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct STrade {
    pub quantity: String,
    pub price: String,
    pub subaccount_id: String,
    fee: String,
    order_hash: String,
    fee_recipient_address: String,
    cid: String,
}