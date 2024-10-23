// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(oneof="event::Item", tags="1, 2, 3, 4, 5")]
    pub item: ::core::option::Option<event::Item>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag="1")]
        EventBatchSpotExecution(super::BatchSpotExecution),
        #[prost(message, tag="2")]
        CreateCampaign(super::CreateCampaign),
        #[prost(message, tag="3")]
        CreateRound(super::CreateRound),
        #[prost(message, tag="4")]
        UpdateCampaign(super::UpdateCampaign),
        #[prost(message, tag="5")]
        UpdateRound(super::UpdateRound),
    }
}
/// Event type: `wasm-incentives-update-campaign`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCampaign {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(float, tag="2")]
    pub new_rewards: f32,
    #[prost(string, tag="3")]
    pub round: ::prost::alloc::string::String,
}
/// Event type: `wasm-incentives-update-round`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRound {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    ///   google.protobuf.Timestamp start_date = 2;
    ///   google.protobuf.Timestamp end_date = 3;
    #[prost(string, tag="2")]
    pub start_date: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub end_date: ::prost::alloc::string::String,
}
/// Event type: `wasm-incentives-create-campaign`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCampaign {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subaccount_suffix: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub rewards: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub round: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub rewards_usd: ::prost::alloc::string::String,
}
/// Event type: `wasm-incentives-create-round`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRound {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    ///   google.protobuf.Timestamp start_date = 2;
    ///   google.protobuf.Timestamp end_date = 3;
    #[prost(string, tag="2")]
    pub start_date: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub end_date: ::prost::alloc::string::String,
}
/// Event type: `injective.exchange.v1beta1.EventBatchSpotExecution`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchSpotExecution {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    #[prost(float, tag="1")]
    pub quantity: f32,
    #[prost(float, tag="2")]
    pub price: f32,
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Field calculated using INJ/USD oracle price
    #[prost(int64, optional, tag="3")]
    pub volume_usd: ::core::option::Option<i64>,
}
// @@protoc_insertion_point(module)
