import { Event } from "./generated_ts/event";
import { EventList } from "./generated_ts/event-list";
import { EventUnion } from "./generated_ts/event-union";
import { GameState } from "./generated_ts/GameState";
import { PlayerJoined } from "./generated_ts/player-joined";
import { Builder, ByteBuffer } from "flatbuffers";

function serializePlayerJoined(): Uint8Array {
    let builder = new Builder(1024);
    let nameOffset = builder.createString("John");
    let playerJoinedOffset = PlayerJoined.createPlayerJoined(
        builder,
        1,
        nameOffset,
    );
    let eventOffset = Event.createEvent(
        builder,
        EventUnion.PlayerJoined,
        playerJoinedOffset,
    );
    let eventsOffset = EventList.createEventsVector(builder, [eventOffset]);
    let eventList = EventList.createEventList(
        builder,
        eventsOffset,
    );
    builder.finish(eventList);
    return builder.asUint8Array();
}

function advance(gameState: GameState, eventList: EventList) {
    let eventsLength = eventList.eventsLength();
    for (let i = 0; i < eventsLength; i++) {
        let event = eventList.events(i);
        if (!event) continue;
        switch (event.eventType()) {
            case EventUnion.PlayerJoined: // 1
                let playerJoined: PlayerJoined | null = event.event(new PlayerJoined());
                if (!playerJoined) continue;
                let id = playerJoined.id();
                let name = playerJoined.name();
                if (!name) continue;
                gameState.players[id] = {
                    id,
                    name,
                    score: 0,
                };
                break;
            default:
                throw new Error("unreachable");
        }
    }
}

let bytes = serializePlayerJoined();
console.log(bytes);
let byteBuffer = new ByteBuffer(bytes);
let eventList = EventList.getRootAsEventList(byteBuffer);
console.log(eventList);
let gameState: GameState = {
    players: {},
};
advance(gameState, eventList);
console.log(gameState);
