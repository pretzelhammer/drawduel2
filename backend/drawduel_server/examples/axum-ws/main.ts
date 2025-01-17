import WebSocket from 'ws';

const socket = new WebSocket('ws://127.0.0.1:3000/ws');

const processSocket = (): Promise<void> => {
    return new Promise((resolve, reject) => {
        // Event listener for when the connection is opened
        socket.on('open', () => {
            console.log('client connected to server');
            // You can send a message to the server
            socket.ping('ping from client');
            socket.send('Hello from client!');
            setTimeout(() => {
                socket.ping('2nd ping from client');
            }, 1000);
        });

        // Event listener for pings from the server
        socket.on('ping', (data) => {
            console.log('ping from server:', data.toString());
            // don't need to explicitly send pong in response to
            // pings because ws lib does it automatically
            // socket.pong('pong');
        });

        // Event listener for pongs from the server
        socket.on('pong', (data) => {
            console.log('pong from server:', data.toString());
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
        console.error('error occurred:', error);
    }
}

main();
