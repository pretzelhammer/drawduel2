import WebSocket from 'ws';
import { ServerEvents } from './generated_ts/game';

interface PlayerInfo {
    name: string;
    pass: string;
    room: string;
}

interface Player extends PlayerInfo {
    socket: WebSocket;
}

const infos = [
    {
        name: 'alex',
        pass: 'aaa',
        room: 'test-room',
    },
    {
        name: 'bob',
        pass: 'bbb',
        room: 'test-room',
    },
    {
        name: 'charlie',
        pass: 'ccc',
        room: 'test-room',
    },
    {
        name: 'dave',
        pass: 'ddd',
        room: 'test-room',
    },
];

function connectionString(i: PlayerInfo): string {
    return `ws://127.0.0.1:3000/ws?name=${i.name}&pass=${i.pass}&room=${i.room}`;
}

// edit slice(0, n) to control how many players connect
const players: Player[] = infos.slice(0, 4).map(i => ({
    ...i,
    socket: new WebSocket(connectionString(i)),
}));

function processPlayer(player: Player): Promise<void> {
    return new Promise((resolve, reject) => {
        const {
            name,
            pass,
            room,
            socket,
        } = player;

        setTimeout(() => {
            console.log(`${name} closing connection`);
            socket.close();
            resolve();
        }, Math.floor(Math.random() * 10000) + 2000);

        socket.on('open', () => {
            console.log(`${name} connected to ${room}`);
        });

        let pingCount = 0;
        socket.on('ping', (_data) => {
            pingCount++;
            console.log(`${name} got ${pingCount} pings`);
            // don't need to explicitly send pong in response to
            // pings because ws lib does it automatically,
            // e.g. socket.pong(_data);
        });

        let pongCount = 0;
        socket.on('pong', (_data) => {
            pongCount++;
            console.log(`${name} got ${pongCount} pongs`);
        });

        socket.on('message', (data: string | Uint8Array) => {
            if (data instanceof Uint8Array) {
                let events = ServerEvents.decode(data);
                console.log('events from server:', JSON.stringify(events));
            } else {
                console.log('message from server:', data);
            }
        });

        socket.on('error', (error) => {
            console.error(`${name} got error: ${error}`);
            reject(error);
        });

        socket.on('close', () => {
            console.log(`server closed ${name}'s connection to ${room}`);
            resolve();
        });
    });
};

async function main(players: Player[]) {
    try {
        let activePlayers = players.map(processPlayer);
        await Promise.all(activePlayers);
    } catch (error) {
        // no-op, already logged
    }
}

main(players);
