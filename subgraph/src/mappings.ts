import { Protobuf } from "as-proto/assembly";
import { Events as protoEvents} from "./pb/sf/substreams/injective/helix/v1/Events";
import {handleBatchSpotExecution, handleCreateRound, handleCreateCampaign, handleUpdateCampaign} from "./event";

export function handleTriggers(bytes: Uint8Array): void {
  const input = Protobuf.decode<protoEvents>(bytes, protoEvents.decode);

  for (let i = 0; i < input.events.length; i++) {
    let current_event = (input.events[i])
    if (current_event.eventBatchSpotExecution !== null) {
      handleBatchSpotExecution(current_event.eventBatchSpotExecution);
    } else if (current_event.createCampaign !== null) {
      handleCreateCampaign(current_event.createCampaign);
    } else if (current_event.createRound !== null) {
      handleCreateRound(current_event.createRound);
    } else if (current_event.updateCampaign !== null) {
      handleUpdateCampaign(current_event.updateCampaign);
    } else {
      console.error("Unknown event type");
    }
  }
}


