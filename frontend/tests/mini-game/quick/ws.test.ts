import WebSocket from 'ws';
import { describe, test, expect, beforeEach, beforeAll } from 'vitest';
import net from 'net';
import isString from 'lodash-es/isString';
import { Game, ServerEvents } from '../../../src/game/mini/engine';

function log(...args: any[]) {
    for (let arg of args) {
        if (isString(arg)) {
            console.log(arg);
        } else {
            console.dir(arg, { depth: null, colors: true });
        }
    }
}

function connect(name?: string, pass?: string): WebSocket {
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

function expectSetGameEvent(
    playerName?: string,
): (Uint8Array) => [PlayerId, Game] {
    return function (msg: Uint8Array): [PlayerId, Game] {
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
}

type expectEventFn = (PlayerId, Game, Uint8Array) => void;

function expectPlayerJoinEvent(playerName: string): expectEventFn {
    return function (_selfId: PlayerId, _game: Game, msg: Uint8Array) {
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
    };
}

function expectPlayerDisconnectEvent(playerName: string): expectEventFn {
    return function (selfId: PlayerId, game: Game, msg: Uint8Array) {
        let serverEvents = ServerEvents.decode(msg);
        let playerId = Number.parseInt(
            Object.entries(game.players).find(
                ([_, player]) => player.name === playerName,
            )?.[0] || '-1',
            10,
        );
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
    };
}

describe.sequential(
    'mini game ws',
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
            let ws = connect();
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

        test('should receive setGame message after open', () => {
            return new Promise<void>((resolve) => {
                let adam = connect('adam');
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
                let player = connect();
                player.on('message', (msg) => {
                    expect(msg).toBeInstanceOf(Uint8Array);
                    expectSetGameEvent();
                    player.close();
                });
                player.on('close', resolve);
            });
        });

        test('multiple players should see each other', () => {
            return new Promise<void>(async (resolve) => {
                // adam init
                let adamId = -1;
                let adamGame = Game.create();
                let adamExpects = [
                    expectSetGameEvent('adam'),
                    expectPlayerJoinEvent('bob'),
                ];
                let adamMsgs = 0;
                let adamClosed = false;

                // bob init
                let bobId = -1;
                let bobGame = Game.create();
                let bobExpects = [
                    expectSetGameEvent('bob'),
                    expectPlayerDisconnectEvent('adam'),
                ];
                let bobMsgs = 0;
                let bobClosed = false;

                // adam connect
                let adam = connect('adam');
                adam.on('message', (msg) => {
                    // log('adam got msg:', ServerEvents.decode(msg));
                    // console.log('adam msg');
                    if (adamMsgs === 0) {
                        // @ts-ignore
                        let [id, game] = adamExpects[0](msg);
                        adamId = id;
                        adamGame = game;
                    } else {
                        adamExpects[adamMsgs](adamId, adamGame, msg);
                    }
                    adamMsgs += 1;
                    if (adamMsgs >= adamExpects.length) {
                        // console.log('closing adam');
                        adam.close();
                    }
                });
                adam.on('close', () => {
                    adamClosed = true;
                    if (bobClosed) {
                        resolve();
                    }
                });

                await sleep(5);

                // bob connect
                let bob = connect('bob');
                bob.on('message', (msg) => {
                    // log('bob got msg:', ServerEvents.decode(msg));
                    // console.log('bob msg');
                    if (bobMsgs === 0) {
                        // @ts-ignore
                        let [id, game] = bobExpects[0](msg);
                        bobId = id;
                        bobGame = game;
                    } else {
                        bobExpects[bobMsgs](bobId, bobGame, msg);
                    }
                    bobMsgs += 1;
                    if (bobMsgs >= bobExpects.length) {
                        // console.log('closing bob');
                        bob.close();
                    }
                });
                bob.on('close', () => {
                    bobClosed = true;
                    if (adamClosed) {
                        resolve();
                    }
                });
            });
        });
    },
);
