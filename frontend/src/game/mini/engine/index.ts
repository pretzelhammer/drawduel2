import {
    Game,
    Guess,
    Phase,
    Player,
    ServerEvent,
    ServerEvents,
    WordChoice,
} from './mini_game';

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
            drawerScore: 0,
            guesserScore: 0,
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
        if (game.round?.drawer == drawOperation.id) {
            game.round?.drawing.push(drawOperation.drawOp!);
        }
    } else if (serverEvent.playerChooseWord) {
        let playerChooseWord = serverEvent.playerChooseWord;
        if (game.round?.drawer == playerChooseWord.drawer) {
            game.round!.wordChoice = playerChooseWord.choice;
        }
    } else if (serverEvent.playerGuessWord) {
        let playerGuessWord = serverEvent.playerGuessWord;
        if (game.round?.drawer != playerGuessWord.guesser) {
            game.round!.guesses.push(playerGuessWord);
        }
    } else if (serverEvent.newRound) {
        let newRound = serverEvent.newRound;
        if (newRound.roundId != game.round?.roundId) {
            let round = game.round!;
            round.roundId = newRound.roundId;
            round.drawer = newRound.drawer;
            round.easyWord = newRound.easyWord;
            round.hardWord = newRound.hardWord;
            round.phase = Phase.CHOOSE_WORD;
            round.wordChoice = WordChoice.EASY;
            round.drawing = [];
            round.drawingScore = 0;
            round.guessingScore = 0;
            round.guesses = [];
            round.hints = [];
        }
    } else {
        throw new Error('unimplemented server event type');
    }
    return game;
}
