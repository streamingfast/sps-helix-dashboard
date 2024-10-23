import {CreateRound} from "./pb/sf/substreams/injective/helix/v1/CreateRound";
import {Campaign, Round} from "../generated/schema";
import {UpdateCampaign} from "./pb/sf/substreams/injective/helix/v1/UpdateCampaign";
import {BigDecimal, BigInt} from "@graphprotocol/graph-ts";
import {CreateCampaign} from "./pb/sf/substreams/injective/helix/v1/CreateCampaign";
import {BatchSpotExecution} from "./pb/sf/substreams/injective/helix/v1/BatchSpotExecution";

export function handleCreateRound(event: CreateRound): void {
    let entity = new Round(event.id);
    entity.save();
}

export function handleUpdateCampaign(event: UpdateCampaign): void {
    let entity = Campaign.load(event.marketId);
    if (!entity) {
        console.error("Campaign not found");
        return;
    }

    entity.rewards = BigInt.fromString(event.newRewards.toString());

    entity.save();
}

export function handleCreateCampaign(event: CreateCampaign): void {
    let entity = new Campaign(event.marketId);
    let round = Round.load(event.round.toString());
    if (!round) {
        console.error("Round not found");
        return;
    }

    round.campaigns.push(entity.id);
    round.save();

    entity.campaign_id = BigInt.fromString(event.id.toString());
    entity.rewards = BigInt.fromString(event.rewards.toString());
    entity.round = event.round
    entity.save();
}

export function handleBatchSpotExecution(event: BatchSpotExecution): void {
    let entity = Campaign.load(event.marketId.toString());
    if (!entity) {
        console.error("Campaign not found");
        return;
    }

    let round = Round.load(entity.round.toString());
    if (!round) {
        console.error("Round not found");
        return;
    }

    for (let i = 0; i < event.trades.length; i++) {
        let trade = event.trades[i];
        let trade_volume = BigDecimal.fromString(trade.volumeUsd.toString());
        if (entity.total_volume) {
            entity.total_volume = entity.total_volume.plus(trade_volume);
        } else {
            entity.total_volume = trade_volume;
        }
        if (round.total_volume) {
            round.total_volume = round.total_volume.plus(trade_volume);
        } else {
            round.total_volume = trade_volume;
        }
    }

    entity.save();
}