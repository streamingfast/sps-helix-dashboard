mod pb;
pub mod types;

use crate::types::STrade;
use core::panic;
use crate::pb::sf::substreams::injective::helix::v1::{Events as HEvents, Event as HEvent, BatchSpotExecution, UpdateCampaign, UpdateRound, CreateRound, CreateCampaign, event::Item as Item, Trade};
use crate::pb::sf::substreams::cosmos::v1::*;
use pb::sf::substreams::injective::oracle::v1::{SetOraclePrice};
use substreams::errors::Error;
use substreams::prelude::*;
use substreams::store::{
    StoreGetProto, StoreAddInt64, StoreGetArray
};
use serde::{Deserialize};

#[substreams::handlers::map]
pub fn map_contract_events(params: String, events: EventList) -> Result<HEvents, Error> {
    let mut hevents = HEvents::default();

    events.events.into_iter().for_each(|event: Event| {
        if let Some(event_data) = event.event {
            match event_data.r#type.as_str() {
                "wasm-incentives-create-round" => {
                    if let Some(_) = event_data.attributes.iter().find(|attr| {
                        attr.key == "_contract_address" && attr.value == params
                    }) {
                        let mut create_round = CreateRound::default();
                        let _ = event_data.attributes.into_iter().for_each(|attr| {
                             match attr.key.as_str() {
                                 "id" => create_round.id = attr.value,
                                 "start_date" => create_round.start_date = attr.value,
                                 "end_date" => create_round.end_date = attr.value,
                                 _ => {}
                             }
                         });

                        let create_round_event = HEvent{
                            item: Some(Item::CreateRound(create_round))
                        };

                        hevents.events.push(create_round_event);
                    }

                },

                "wasm-incentives-create-campaign" => {
                    let attributes = event_data.attributes;
                    if let Some(_) = attributes.iter().find(|&attr| {
                        attr.key == "_contract_address" && attr.value == params
                    }) {
                        let mut create_campaign = CreateCampaign::default();
                        let _ = attributes.into_iter().for_each(|attr| {
                            match attr.key.as_str() {
                                "id" => create_campaign.id = attr.value,
                                "market_id" => create_campaign.market_id = attr.value,
                                "subaccount_suffix" => create_campaign.subaccount_suffix = attr.value,
                                "rewards" => create_campaign.rewards = attr.value.parse().unwrap(),
                                _ => {}
                            }
                        });

                        let create_campaign_event = HEvent{
                            item: Some(Item::CreateCampaign(create_campaign))
                        };

                        hevents.events.push(create_campaign_event);
                    }

                },

                "injective.exchange.v1beta1.EventBatchSpotExecution" => {
                    let attributes = event_data.attributes;
                    if let Some(_) = attributes.iter().find(|&attr| {
                        attr.key == "_contract_address" && attr.value == params
                    }) {
                        let mut batch_spot_execution = BatchSpotExecution::default();
                        let _ = attributes.into_iter().for_each(|attr| {
                            match attr.key.as_str() {
                                "market_id" => batch_spot_execution.market_id = attr.value,
                                "trades" => {
                                    let s_trades :Vec<STrade>= serde_json::from_str(&attr.value).unwrap();
                                    let mut trades: Vec<Trade> = Vec::new();
                                    
                                    s_trades.into_iter().for_each(|s_trade| {
                                        let trade = Trade{
                                            quantity: s_trade.quantity.parse().unwrap(),
                                            price: s_trade.price.parse().unwrap(),
                                            subaccount_id: s_trade.subaccount_id,
                                            volume_usd: None,
                                        };

                                        trades.push(trade);
                                    });

                                    batch_spot_execution.trades = trades;
                                },
                                _ => {}
                            }
                        });
                    }
                },

                "wasm-incentives-update-round" => {
                    if let Some(_) = event_data.attributes.iter().find(|attr| {
                        attr.key == "_contract_address" && attr.value == params
                    }) {
                        let update_round = UpdateRound::default();
                    }
                },

                "wasm-incentives-update-campaign" => {
                    if let Some(_) = event_data.attributes.iter().find(|attr| {
                        attr.key == "_contract_address" && attr.value == params
                    }) {
                       let update_campaign = UpdateCampaign::default();
                    }

                },

                _ => {

                }
            }
        }
    });

    Ok(hevents)
}

#[substreams::handlers::store]
pub fn store_round_info(events: HEvents, store: StoreSetString) {
    events.events.iter().for_each(|event: &HEvent| {
        match event.item.as_ref() {
            Some(item) => match item {
                Item::CreateRound(create_round)  => {
                    let key = String::from("campaign");

                    // Delete previous round
                    store.delete_prefix(0, &key);
                    store.set(0, "current_round", &create_round.id);
                },
                Item::CreateCampaign(create_campaign) => {
                    store.set(0, format!("campaign:{}", create_campaign.market_id), &create_campaign.subaccount_suffix);
                },
                _ => {

                }
            }
            None => {}
        }
    });
}

#[substreams::handlers::map]
pub fn map_filtered_trades(events: HEvents, store_round_info: StoreGetString, store_pyth_price: StoreGetProto<SetOraclePrice>) -> Result<HEvents, Error> {
    let mut filtered_events = HEvents::default();

    events.events.into_iter().for_each(|event: HEvent| {
        match event.item.clone().as_ref() {
            Some(item) => match item {
                Item::CreateRound(_)  => {
                    filtered_events.events.push(event);
                },

                Item::CreateCampaign(create_campaign) => {
                    if store_round_info.get_last("current_round").unwrap() == create_campaign.round {
                        filtered_events.events.push(event);
                    }
                },

                Item::UpdateCampaign(update_campaign) => {
                    if store_round_info.get_last("current_round").unwrap() == update_campaign.round {
                        if store_round_info.has_last(format!("round:campaign:{}", update_campaign.market_id.as_str())) {
                            filtered_events.events.push(event);
                        }
                    }
                },

                Item::EventBatchSpotExecution(batch_spot_execution) => {
                    // filters batch_spot_execution trades without `subaccount_suffix` corresponding to the current market
                    let mut update_trades: Vec<Trade> = Vec::new();
                    batch_spot_execution.trades.iter().for_each(|trade| {
                        if store_round_info.has_last(format!("round:campaign:{}", batch_spot_execution.market_id)) {
                            if let Some(subaccount_suffix) = store_round_info.get_last(format!("round:campaign:{}", batch_spot_execution.market_id)){
                                if trade.subaccount_id.ends_with(&subaccount_suffix) {
                                    let inj_usd_set_price = store_pyth_price.get_last("oracle_price:0x7a5bc1d2b56ad029048cd63964b3ad2776eadf812edc1a43a31406cb54bff592").unwrap();
                                    let inj_usd_price = inj_usd_set_price.price_state.unwrap().price;

                                    let mut update_trade = trade.clone();
                                    let volume: f64 = (trade.quantity as f64) * (trade.price as f64);;
                                    update_trade.volume_usd = Some((volume * inj_usd_price) as i64);

                                    update_trades.push(update_trade);
                                }
                            };
                        }
                    });

                    let mut updated_batch_spot_execution = BatchSpotExecution{
                        market_id: batch_spot_execution.market_id.clone(),
                        trades: update_trades
                    };

                },

                _ => {}
            }
            None => {}
        }
    });


    Ok(filtered_events)
}


