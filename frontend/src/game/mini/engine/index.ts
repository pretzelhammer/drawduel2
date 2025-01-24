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
        let increaseScore = serverEvent.playerIncreaseScore;
        if (game.players[increaseScore.id]) {
            game.players[increaseScore.id].score += increaseScore.increaseBy;
        }
    } else if (serverEvent.playerIncreaseGuesserScore) {
        let increaseScore = serverEvent.playerIncreaseGuesserScore;
        if (game.players[increaseScore.id]) {
            game.players[increaseScore.id].guesserScore +=
                increaseScore.increaseBy;
        }
    } else if (serverEvent.playerIncreaseDrawerScore) {
        let increaseScore = serverEvent.playerIncreaseDrawerScore;
        if (game.players[increaseScore.id]) {
            game.players[increaseScore.id].drawerScore +=
                increaseScore.increaseBy;
        }
    } else if (serverEvent.playerDrawOp) {
        let drawOperation = serverEvent.playerDrawOp;
        if (game.drawing!.byPlayer == drawOperation.id) {
            game.drawing!.drawOps.push(drawOperation.drawOp!);
        }
    } else {
        // no-op
    }
    return game;
}
