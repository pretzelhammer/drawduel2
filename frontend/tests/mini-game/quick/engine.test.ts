import { describe, test, expect } from 'vitest';
import {
    ColorType,
    Game,
    Phase,
    ServerEvent,
    ServerEvents,
    WordChoice,
    advanceAllGame,
} from 'src/game/mini/engine';
import { server_advance } from 'tests/wasm/drawduel_wasm';

import isString from 'lodash-es/isString';
function log(...args: any[]) {
    for (let arg of args) {
        if (isString(arg)) {
            console.log(arg);
        } else {
            console.dir(arg, { depth: null, colors: true });
        }
    }
}

function expectServerClientGamesInSync(serverEvent: ServerEvent, game: Game) {
    // log({
    //     game,
    //     sourceServerEvent: serverEvent,
    // });
    let next_state = server_advance(
        ServerEvent.encode(serverEvent).finish(),
        Game.encode(game).finish(),
    );
    if (next_state) {
        let serverEvents = ServerEvents.decode(
            new Uint8Array(next_state.apply_events),
        );
        let serverGame = Game.decode(new Uint8Array(next_state.next_game));
        let clientGame = advanceAllGame(serverEvents, game);
        // log({
        //     returnedServerEvents: serverEvents,
        //     clientGame,
        //     serverGame,
        // });
        expect(clientGame).toStrictEqual(serverGame);
    }
}

function newRound(): object {
    return {
        roundId: 0,
        phase: Phase.CHOOSE_WORD,
        drawer: 0,
        drawing: [],
        easyWord: 'easy',
        hardWord: 'hard',
        wordChoice: WordChoice.EASY,
        drawingScore: 0,
        guessingScore: 0,
        guesses: [],
        hints: [],
        endsAt: 0,
    };
}

function newGame(): Game {
    return Game.fromPartial({
        players: {},
        // round: newRound(),
    });
}

function onePlayerGame(): Game {
    return Game.fromPartial({
        players: {
            0: {
                name: 'adam',
                score: 0,
                connected: true,
                drawerScore: 0,
                guesserScore: 0,
            },
        },
        // round: newRound(),
    });
}

function twoPlayerGame(): Game {
    return Game.fromPartial({
        players: {
            0: {
                name: 'adam',
                score: 0,
                connected: true,
                drawerScore: 0,
                guesserScore: 0,
            },
            1: {
                name: 'bob',
                score: 0,
                connected: true,
                drawerScore: 0,
                guesserScore: 0,
            },
        },
        round: newRound(),
    });
}

describe(
    'mini game engine',
    {
        timeout: 500,
    },
    () => {
        // new

        test('player joins (new game)', () => {
            let game = newGame();
            let playerJoin = ServerEvent.fromPartial({
                playerJoin: {
                    id: 0,
                    name: 'adam',
                },
            });
            expectServerClientGamesInSync(playerJoin, game);
        });

        test('player leaves (new game)', () => {
            let game = newGame();
            let playerLeave = ServerEvent.fromPartial({
                playerLeave: {
                    id: 0,
                },
            });
            expectServerClientGamesInSync(playerLeave, game);
        });

        test('player connects (new game)', () => {
            let game = newGame();
            let playerConnect = ServerEvent.fromPartial({
                playerConnect: {
                    id: 0,
                },
            });
            expectServerClientGamesInSync(playerConnect, game);
        });

        test('player disconnects (new game)', () => {
            let game = newGame();
            let playerDisconnect = ServerEvent.fromPartial({
                playerDisconnect: {
                    id: 0,
                },
            });
            expectServerClientGamesInSync(playerDisconnect, game);
        });

        test('player renames (new game)', () => {
            let game = newGame();
            let playerRename = ServerEvent.fromPartial({
                playerRename: {
                    id: 0,
                    name: 'whatever',
                },
            });
            expectServerClientGamesInSync(playerRename, game);
        });

        test('player increases score (new game)', () => {
            let game = newGame();
            let playerIncreaseScore = ServerEvent.fromPartial({
                playerIncreaseScore: {
                    id: 0,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseScore, game);
        });

        test('player increases drawer score (new game)', () => {
            let game = newGame();
            let playerIncreaseDrawerScore = ServerEvent.fromPartial({
                playerIncreaseDrawerScore: {
                    id: 0,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseDrawerScore, game);
        });

        test('player increases guesser score (new game)', () => {
            let game = newGame();
            let playerIncreaseGuesserScore = ServerEvent.fromPartial({
                playerIncreaseGuesserScore: {
                    id: 0,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseGuesserScore, game);
        });

        test('player draw op (new game)', () => {
            let game = newGame();
            let playerDrawOp = ServerEvent.fromPartial({
                playerDrawOp: {
                    id: 0,
                    drawOp: {
                        startStroke: {
                            colorType: ColorType.PRIMARY,
                            x: 0.5,
                            y: 0.5,
                        },
                    },
                },
            });
            expectServerClientGamesInSync(playerDrawOp, game);
        });

        test('player choose word (new game)', () => {
            let game = newGame();
            let playerChooseWord = ServerEvent.fromPartial({
                playerChooseWord: {
                    drawer: 0,
                    choice: WordChoice.HARD,
                },
            });
            expectServerClientGamesInSync(playerChooseWord, game);
        });

        test('player incorrect guess word (new game)', () => {
            let game = newGame();
            let playerGuessWord = ServerEvent.fromPartial({
                playerGuessWord: {
                    guesser: 1,
                    guess: 'incorrect',
                    afterDrawOps: 0,
                },
            });
            expectServerClientGamesInSync(playerGuessWord, game);
        });

        // one player

        test('player joins (one player)', () => {
            let game = onePlayerGame();
            let playerJoin = ServerEvent.fromPartial({
                playerJoin: {
                    id: 1,
                    name: 'bob',
                },
            });
            expectServerClientGamesInSync(playerJoin, game);
        });

        test('player leaves (one player)', () => {
            let game = onePlayerGame();
            let playerLeave = ServerEvent.fromPartial({
                playerLeave: {
                    id: 0,
                },
            });
            expectServerClientGamesInSync(playerLeave, game);
        });

        test('player connects (one player)', () => {
            let game = onePlayerGame();
            let playerConnect = ServerEvent.fromPartial({
                playerConnect: {
                    id: 0,
                },
            });
            expectServerClientGamesInSync(playerConnect, game);
        });

        test('player disconnects (one player)', () => {
            let game = onePlayerGame();
            let playerDisconnect = ServerEvent.fromPartial({
                playerDisconnect: {
                    id: 0,
                },
            });
            expectServerClientGamesInSync(playerDisconnect, game);
        });

        test('player renames (one player)', () => {
            let game = onePlayerGame();
            let playerRename = ServerEvent.fromPartial({
                playerRename: {
                    id: 0,
                    name: 'adam2',
                },
            });
            expectServerClientGamesInSync(playerRename, game);
        });

        test('player increases score (one player)', () => {
            let game = onePlayerGame();
            let playerIncreaseScore = ServerEvent.fromPartial({
                playerIncreaseScore: {
                    id: 0,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseScore, game);
        });

        test('player increases drawer score (one player)', () => {
            let game = onePlayerGame();
            let playerIncreaseDrawerScore = ServerEvent.fromPartial({
                playerIncreaseDrawerScore: {
                    id: 0,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseDrawerScore, game);
        });

        test('player increases guesser score (one player)', () => {
            let game = onePlayerGame();
            let playerIncreaseGuesserScore = ServerEvent.fromPartial({
                playerIncreaseGuesserScore: {
                    id: 0,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseGuesserScore, game);
        });

        test('player draw op (one player)', () => {
            let game = onePlayerGame();
            let playerDrawOp = ServerEvent.fromPartial({
                playerDrawOp: {
                    id: 0,
                    drawOp: {
                        startStroke: {
                            colorType: ColorType.PRIMARY,
                            x: 0.5,
                            y: 0.5,
                        },
                    },
                },
            });
            expectServerClientGamesInSync(playerDrawOp, game);
        });

        test('player choose word (one player)', () => {
            let game = onePlayerGame();
            let playerChooseWord = ServerEvent.fromPartial({
                playerChooseWord: {
                    drawer: 0,
                    choice: WordChoice.HARD,
                },
            });
            expectServerClientGamesInSync(playerChooseWord, game);
        });

        test('player incorrect guess word (one player)', () => {
            let game = onePlayerGame();
            let playerGuessWord = ServerEvent.fromPartial({
                playerGuessWord: {
                    guesser: 1,
                    guess: 'incorrect',
                    afterDrawOps: 0,
                },
            });
            expectServerClientGamesInSync(playerGuessWord, game);
        });

        // two player

        test('player joins (two player)', () => {
            let game = twoPlayerGame();
            let playerJoin = ServerEvent.fromPartial({
                playerJoin: {
                    id: 2,
                    name: 'caleb',
                },
            });
            expectServerClientGamesInSync(playerJoin, game);
        });

        test('player leaves (two player)', () => {
            let game = twoPlayerGame();
            let playerLeave = ServerEvent.fromPartial({
                playerLeave: {
                    id: 1,
                },
            });
            expectServerClientGamesInSync(playerLeave, game);
        });

        test('player connects (two player)', () => {
            let game = twoPlayerGame();
            let playerConnect = ServerEvent.fromPartial({
                playerConnect: {
                    id: 1,
                },
            });
            expectServerClientGamesInSync(playerConnect, game);
        });

        test('player disconnects (two player)', () => {
            let game = twoPlayerGame();
            let playerDisconnect = ServerEvent.fromPartial({
                playerDisconnect: {
                    id: 1,
                },
            });
            expectServerClientGamesInSync(playerDisconnect, game);
        });

        test('player renames (two player)', () => {
            let game = twoPlayerGame();
            let playerRename = ServerEvent.fromPartial({
                playerRename: {
                    id: 1,
                    name: 'bob2',
                },
            });
            expectServerClientGamesInSync(playerRename, game);
        });

        test('player increases score (two player)', () => {
            let game = twoPlayerGame();
            let playerIncreaseScore = ServerEvent.fromPartial({
                playerIncreaseScore: {
                    id: 1,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseScore, game);
        });

        test('player increases drawer score (two player)', () => {
            let game = twoPlayerGame();
            let playerIncreaseDrawerScore = ServerEvent.fromPartial({
                playerIncreaseDrawerScore: {
                    id: 1,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseDrawerScore, game);
        });

        test('player increases guesser score (two player)', () => {
            let game = twoPlayerGame();
            let playerIncreaseGuesserScore = ServerEvent.fromPartial({
                playerIncreaseGuesserScore: {
                    id: 1,
                    increaseBy: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseGuesserScore, game);
        });

        test('player draw op (two player)', () => {
            let game = twoPlayerGame();
            let playerDrawOp = ServerEvent.fromPartial({
                playerDrawOp: {
                    id: 0,
                    drawOp: {
                        startStroke: {
                            colorType: ColorType.PRIMARY,
                            x: 0.5,
                            y: 0.5,
                        },
                    },
                },
            });
            expectServerClientGamesInSync(playerDrawOp, game);
        });

        test('player choose word (two player)', () => {
            let game = twoPlayerGame();
            let playerChooseWord = ServerEvent.fromPartial({
                playerChooseWord: {
                    drawer: 0,
                    choice: WordChoice.HARD,
                },
            });
            expectServerClientGamesInSync(playerChooseWord, game);
        });

        test('player incorrect guess word (two player)', () => {
            let game = twoPlayerGame();
            let playerGuessWord = ServerEvent.fromPartial({
                playerGuessWord: {
                    guesser: 1,
                    guess: 'incorrect',
                    afterDrawOps: 0,
                },
            });
            expectServerClientGamesInSync(playerGuessWord, game);
        });
    },
);
