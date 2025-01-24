import WebSocket from 'ws';
import { describe, test, expect, beforeAll } from 'vitest';
import net from 'net';
// import isString from 'lodash-es/isString';
import {
    advanceAllGame,
    ClientEvent,
    Game,
    ServerEvents,
} from 'src/game/mini/engine';

// function log(...args: any[]) {
//     for (let arg of args) {
//         if (isString(arg)) {
//             console.log(arg);
//         } else {
//             console.dir(arg, { depth: null, colors: true });
//         }
//     }
// }

function getSocket(name?: string, pass?: string): WebSocket {
    pass = pass || Math.random().toString(32).slice(2, 8);
    let url = `ws://localhost:42069/mini-game-ws?pass=${pass}`;
    if (name) {
        url += `&name=${name}`;
    }
    return new WebSocket(url);
}

function sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

type PlayerId = number;

interface ExpectSetGameEvent {
    action: 'expectSetGameEvent';
    fn: expectSetGameEventFn;
}
type expectSetGameEventFn = (msg: Uint8Array) => [PlayerId, Game];

function getPlayerIdByName(playerName: string, game: Game): PlayerId {
    return Number.parseInt(
        Object.entries(game.players).find(
            ([_, player]) => player.name === playerName,
        )?.[0] || '-1',
        10,
    );
}

function expectSetGameEvent(playerName?: string): Action {
    let fn = function (msg: Uint8Array): [PlayerId, Game] {
        let serverEvents = ServerEvents.decode(msg);
        let playerId = serverEvents.events[0].setGame?.playerId;
        expect(playerId).toBeDefined();
        expect(playerId).toBeGreaterThanOrEqual(0);
        playerId = playerId!;
        playerName =
            playerName ||
            serverEvents.events[0].setGame?.game?.players[playerId].name;
        expect(playerName).toBeTruthy();
        playerName = playerName!;
        expect(serverEvents).toMatchObject(
            ServerEvents.fromPartial({
                events: [
                    {
                        setGame: {
                            playerId: playerId,
                            game: {
                                players: {
                                    [playerId]: {
                                        name: playerName,
                                        score: 0,
                                        drawerScore: 0,
                                        guesserScore: 0,
                                        connected: true,
                                    },
                                },
                            },
                        },
                    },
                ],
            }),
        );
        return [playerId, serverEvents.events[0].setGame!.game!];
    };
    return {
        action: 'expectSetGameEvent',
        fn,
    };
}

interface ExpectEvent {
    action: 'expectEvent';
    fn: expectEventFn;
}
type expectEventFn = (id: PlayerId, game: Game, msg: Uint8Array) => Game;

function expectPlayerJoinEvent(playerName: string): Action {
    let fn = function (_id: PlayerId, game: Game, msg: Uint8Array) {
        let serverEvents = ServerEvents.decode(msg);
        expect(serverEvents).toEqual(
            ServerEvents.fromPartial({
                events: [
                    {
                        playerJoin: {
                            id: expect.any(Number),
                            name: playerName,
                        },
                    },
                ],
            }),
        );
        return advanceAllGame(serverEvents, game);
    };
    return {
        action: 'expectEvent',
        fn,
    };
}

function expectPlayerDisconnectEvent(playerName: string): Action {
    let fn = function (_id: PlayerId, game: Game, msg: Uint8Array) {
        let serverEvents = ServerEvents.decode(msg);
        let playerId = getPlayerIdByName(playerName, game);
        expect(serverEvents).toEqual(
            ServerEvents.fromPartial({
                events: [
                    {
                        playerDisconnect: {
                            id: playerId,
                        },
                    },
                ],
            }),
        );
        return advanceAllGame(serverEvents, game);
    };
    return {
        action: 'expectEvent',
        fn,
    };
}

function expectPlayerRename(prevName: string, nextName: string): Action {
    let fn = function (_id: PlayerId, game: Game, msg: Uint8Array) {
        let serverEvents = ServerEvents.decode(msg);
        let playerId = getPlayerIdByName(prevName, game);
        expect(serverEvents).toEqual(
            ServerEvents.fromPartial({
                events: [
                    {
                        playerRename: {
                            id: playerId,
                            name: nextName,
                        },
                    },
                ],
            }),
        );
        return advanceAllGame(serverEvents, game);
    };
    return {
        action: 'expectEvent',
        fn,
    };
}

function expectPlayerIncreaseScore(name: string, score: number): Action {
    let fn = function (_id: PlayerId, game: Game, msg: Uint8Array) {
        let serverEvents = ServerEvents.decode(msg);
        let playerId = getPlayerIdByName(name, game);
        expect(serverEvents).toEqual(
            ServerEvents.fromPartial({
                events: [
                    {
                        playerIncreaseScore: {
                            id: playerId,
                            increaseBy: score,
                        },
                    },
                ],
            }),
        );
        return advanceAllGame(serverEvents, game);
    };
    return {
        action: 'expectEvent',
        fn,
    };
}

interface SendMsg {
    action: 'sendMsg';
    fn: sendMsgFn;
}
type sendMsgFn = (id: PlayerId, game: Game) => ClientEvent;

function sendPlayerRename(rename: string): Action {
    let fn = function (_id: PlayerId, _game: Game): ClientEvent {
        return ClientEvent.fromPartial({
            rename: {
                name: rename,
            },
        });
    };
    return {
        action: 'sendMsg',
        fn,
    };
}

function sendPlayerIncreaseScore(score: number): Action {
    let fn = function (_id: PlayerId, _game: Game): ClientEvent {
        return ClientEvent.fromPartial({
            increaseScore: {
                increaseBy: score,
            },
        });
    };
    return {
        action: 'sendMsg',
        fn,
    };
}

type Action = ExpectSetGameEvent | ExpectEvent | SendMsg;
type ActionSequence = Action[];

function playGame(playerName: string, actions: ActionSequence): Promise<void> {
    return new Promise((resolve) => {
        let ws = getSocket(playerName);
        let playerId = -1;
        let playerGame = Game.create();
        let index = 0;
        ws.on('message', async (msg) => {
            expect(msg).toBeInstanceOf(Uint8Array);
            if (msg instanceof Uint8Array) {
                // log(`${playerName} got msg:`, ServerEvents.decode(msg));
                let action = actions[index];
                if (action.action === 'expectSetGameEvent') {
                    let [id, game] = action.fn(msg);
                    playerId = id;
                    playerGame = game;
                } else if (action.action === 'expectEvent') {
                    playerGame = action.fn(playerId, playerGame, msg);
                }
                index += 1;
                if (index >= actions.length) {
                    // log(`${playerName} closing`);
                    ws.close();
                    return;
                }
                while (index < actions.length) {
                    let peekAction = actions[index];
                    if (peekAction.action === 'sendMsg') {
                        let clientEvent = peekAction.fn(playerId, playerGame);
                        // log(`${playerName} sending msg:`, clientEvent);
                        ws.send(ClientEvent.encode(clientEvent).finish());
                        index += 1;
                    } else {
                        break;
                    }
                }
            }
        });
        ws.on('close', () => {
            // log(`${playerName} closed`);
            resolve();
        });
    });
}

describe.sequential(
    'mini game ws (quick tests)',
    {
        timeout: 1000,
    },
    () => {
        beforeAll(async () => {
            let isServerRunning = await new Promise((resolve) => {
                const socket = net.createConnection(
                    { host: '127.0.0.1', port: 42069 },
                    () => {
                        socket.end();
                        resolve(true);
                    },
                );
                socket.on('error', () => {
                    resolve(false);
                });
            });

            if (!isServerRunning) {
                throw new Error('No game server running on 127.0.0.1:42069');
            }
        });

        test('can open & close connection', () => {
            let ws = getSocket();
            return new Promise<void>((resolve) => {
                ws.on('open', () => {
                    expect(ws.readyState).toBe(WebSocket.OPEN);
                    ws.close();
                });
                ws.on('close', (code) => {
                    expect(ws.readyState).toBe(WebSocket.CLOSED);
                    // 1000 = normal close code
                    // 1006 = no close code sent
                    expect(code).toBe(1006);
                    resolve();
                });
            });
        });

        test('pings responded to with pongs', () => {
            let ws = getSocket();
            return new Promise<void>((resolve) => {
                ws.on('open', async () => {
                    ws.ping();
                    await sleep(5);
                    ws.ping();
                });
                let expectPongs = 2;
                let gotPongs = 0;
                ws.on('pong', () => {
                    gotPongs += 1;
                    if (gotPongs >= expectPongs) {
                        ws.close();
                    }
                });
                ws.on('close', () => {
                    resolve();
                });
            });
        });

        test('should receive setGame message after open', () => {
            return new Promise<void>((resolve) => {
                let adam = getSocket('adam');
                adam.on('message', (msg) => {
                    expect(msg).toBeInstanceOf(Uint8Array);
                    expectSetGameEvent('adam');
                    adam.close();
                });
                adam.on('close', resolve);
            });
        });

        test('get name assigned by server if none provided', () => {
            return new Promise<void>((resolve) => {
                let player = getSocket();
                player.on('message', (msg) => {
                    expect(msg).toBeInstanceOf(Uint8Array);
                    expectSetGameEvent();
                    player.close();
                });
                player.on('close', resolve);
            });
        });

        test('two players should see each other', async () => {
            let players: Promise<void>[] = [];
            players.push(
                playGame('adam', [
                    expectSetGameEvent('adam'),
                    expectPlayerJoinEvent('bob'),
                ]),
            );
            await sleep(5);
            players.push(
                playGame('bob', [
                    expectSetGameEvent('bob'),
                    expectPlayerDisconnectEvent('adam'),
                ]),
            );
            await Promise.all(players);
        });

        test('two players play game', async () => {
            let players: Promise<void>[] = [];
            players.push(
                playGame('adam', [
                    expectSetGameEvent('adam'),
                    expectPlayerJoinEvent('bob'),
                    sendPlayerRename('adam2'),
                    expectPlayerRename('adam', 'adam2'),
                    expectPlayerIncreaseScore('bob', 123),
                ]),
            );
            await sleep(5);
            players.push(
                playGame('bob', [
                    expectSetGameEvent('bob'),
                    expectPlayerRename('adam', 'adam2'),
                    sendPlayerIncreaseScore(123),
                    expectPlayerIncreaseScore('bob', 123),
                    expectPlayerDisconnectEvent('adam2'),
                ]),
            );
            await Promise.all(players);
        });
    },
);
