mod pb;

use core::panic;

use crate::pb::sf::substreams::cosmos::v1::*;
use num_traits::Float;
use pb::sf::substreams::injective::oracle::v1::{SetOraclePrice, PriceState};
use substreams::errors::Error;
use substreams::log;
use substreams::prelude::*;
use substreams::store::{
    StoreSetProto,
};

#[substreams::handlers::map]
pub fn map_set_oracle_prices(events: EventList, store_oracle_price: StoreGetProto<SetOraclePrice>) -> Result<SetOraclePrice, Error> {

}


#[substreams::handlers::store]
pub fn map_set_oracle_prices(events: EventList, store_oracle_price: StoreGetProto<SetOraclePrice>) -> Result<SetOraclePrice, Error> {

}
