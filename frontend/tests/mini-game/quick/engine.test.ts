import { describe, test, expect, beforeAll } from 'vitest';
import {
    Game,
    Player,
    ServerEvent,
    ServerEvents,
    advanceAllGame,
    advanceGame,
} from '../../../src/game/mini/engine';
import { client_advance, server_advance } from '../../wasm/drawduel_wasm';
import cloneDeep from 'lodash-es/cloneDeep';
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
        // log(clientGame, serverGame);
        expect(clientGame).toStrictEqual(serverGame);
    }
}

function newGame() {
    return Game.create();
}

function onePlayerGame() {
    return Game.fromPartial({
        players: {
            0: {
                name: 'adam',
                score: 0,
                connected: true,
            },
        },
    });
}

function twoPlayerGame() {
    return Game.fromPartial({
        players: {
            0: {
                name: 'adam',
                score: 0,
                connected: true,
            },
            1: {
                name: 'bob',
                score: 0,
                connected: true,
            },
        },
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
                    score: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseScore, game);
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
                    score: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseScore, game);
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
                    score: 100,
                },
            });
            expectServerClientGamesInSync(playerIncreaseScore, game);
        });
    },
);
