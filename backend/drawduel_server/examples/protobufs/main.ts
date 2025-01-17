import { GameState } from "./generated_ts/GameState";
import { Event, EventList, PlayerJoined } from "./generated_ts/GameEvents";

function serializePlayerJoined(): Uint8Array {
    const eventList = EventList.fromPartial({
        events: [
            {
                playerJoined: {
                    id: 0,
                    name: "John",
                },
            },
        ],
    });
    return EventList.encode(eventList).finish();
}

function advance(gameState: GameState, eventList: EventList) {
    for (const event of eventList.events) {
        if (event.playerJoined) {
            const { id, name } = event.playerJoined;
            gameState.players[id] = {
                id,
                name,
                score: 0,
            };
        }
    }
}

let bytes = serializePlayerJoined();
console.log(bytes);
let eventList = EventList.decode(bytes);
console.log(eventList);
let gameState: GameState = {
    players: {},
};
advance(gameState, eventList);
console.log(gameState);
