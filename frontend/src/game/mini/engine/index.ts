import { Game, Player, ServerEvent, ServerEvents } from './mini_game';

export * from './mini_game';

export function newGame(): Game {
    return Game.create();
}

export function advanceAllGame(serverEvents: ServerEvents, game: Game): Game {
    for (let serverEvent of serverEvents.events) {
        game = advanceGame(serverEvent, game);
    }
    return game;
}

export function advanceGame(serverEvent: ServerEvent, game: Game): Game {
    if (serverEvent.setGame) {
        return serverEvent.setGame.game!;
    } else if (serverEvent.playerJoin) {
        let playerJoin = serverEvent.playerJoin;
        game.players[playerJoin.id] = Player.fromPartial({
            name: playerJoin.name,
            score: 0,
            connected: true,
        });
    } else if (serverEvent.playerLeave) {
        let playerLeave = serverEvent.playerLeave;
        delete game.players[playerLeave.id];
    } else if (serverEvent.playerConnect) {
        let playerConnect = serverEvent.playerConnect;
        if (game.players[playerConnect.id]) {
            game.players[playerConnect.id].connected = true;
        }
    } else if (serverEvent.playerDisconnect) {
        let playerDisconnect = serverEvent.playerDisconnect;
        if (game.players[playerDisconnect.id]) {
            game.players[playerDisconnect.id].connected = false;
        }
    } else if (serverEvent.playerRename) {
        let playerRename = serverEvent.playerRename;
        if (game.players[playerRename.id]) {
            game.players[playerRename.id].name = playerRename.name;
        }
    } else if (serverEvent.playerIncreaseScore) {
        let playerIncreaseScore = serverEvent.playerIncreaseScore;
        if (game.players[playerIncreaseScore.id]) {
            game.players[playerIncreaseScore.id].score =
                playerIncreaseScore.score;
        }
    } else {
        // no-op
    }
    return game;
}
