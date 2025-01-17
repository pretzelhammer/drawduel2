import WebSocket from 'ws';

const socket = new WebSocket('ws://127.0.0.1:3000/ws?name=bob&pass=yolo');

const processSocket = (): Promise<void> => {
    return new Promise((resolve, reject) => {
        // Event listener for when the connection is opened
        socket.on('open', () => {
            console.log('client connected to server');
            socket.send('hello from client!');
        });

        // Event listener for pings from the server
        let pingCount = 0;
        socket.on('ping', (data) => {
            pingCount++;
            console.log(`ping ${pingCount} from server: `, data.toString());
            // don't need to explicitly send pong in response to
            // pings because ws lib does it automatically
            // socket.pong('pong');
        });

        // Event listener for pongs from the server
        socket.on('pong', (data) => {
            console.log('pong from server: ', data.toString());
            // don't need to explicitly send pong in response to
            // pings because ws lib does it automatically
            // socket.pong('pong');
        });

        // Event listener for receiving messages from the server
        socket.on('message', (data) => {
            console.log('message from server: ', data.toString());
        });

        // Event listener for errors
        socket.on('error', (error) => {
            console.error('ws error: ', error);
            reject(error);
        });

        // Event listener for when the connection is closed
        socket.on('close', () => {
            console.log('ws connection closed');
            resolve();
        });
    });
};

async function main() {
    try {
        await processSocket();
    } catch (error) {
        // no-op, already logged
    }
}

main();
