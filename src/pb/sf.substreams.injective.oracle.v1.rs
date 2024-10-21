// @generated
/// Define the message for SetOraclePrice
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetOraclePrice {
    #[prost(string, tag="1")]
    pub price_id: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub ema_price: f64,
    #[prost(double, tag="3")]
    pub ema_conf: f64,
    #[prost(double, tag="4")]
    pub conf: f64,
    #[prost(int64, tag="5")]
    pub publish_time: i64,
    #[prost(message, optional, tag="6")]
    pub price_state: ::core::option::Option<PriceState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceState {
    #[prost(double, tag="1")]
    pub price: f64,
    #[prost(double, tag="2")]
    pub cumulative_price: f64,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
// @@protoc_insertion_point(module)
