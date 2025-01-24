/* eslint-disable @typescript-eslint/no-unused-vars */
/* @ts-ignore */

import WebSocket from 'ws';
import { describe, test, beforeAll } from 'vitest';
import net from 'net';
// import isString from 'lodash-es/isString';

// function log(...args: any[]) {
//     for (let arg of args) {
//         if (isString(arg)) {
//             console.log(arg);
//         } else {
//             console.dir(arg, { depth: null, colors: true });
//         }
//     }
// }

function connect(name?: string, pass?: string): WebSocket {
    pass = pass || Math.random().toString(32).slice(2, 8);
    let url = `ws://localhost:42069/mini-game-ws?pass=${pass}`;
    if (name) {
        url += `&name=${name}`;
    }
    return new WebSocket(url);
}

describe.sequential('mini game ws (long tests)', () => {
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

    test(
        'inactive connection receives pings',
        {
            // should get 2 pings within
            // 15 secs in dev
            timeout: 15_000,
        },
        () => {
            let ws = connect();
            return new Promise<void>((resolve) => {
                let expectedPings = 2;
                let gotPings = 0;
                ws.on('ping', () => {
                    gotPings += 1;
                    if (gotPings >= expectedPings) {
                        ws.close();
                    }
                });
                ws.on('close', () => {
                    resolve();
                });
            });
        },
    );
});
