import ws from 'k6/ws';

import { check, sleep } from 'k6';

export const options = {
    discardResponseBodies: true,
    scenarios: {
        contacts: {
            executor: 'ramping-vus',
            startVUs: 0,
            stages: [
                { duration: '20s', target: 1500 },
                { duration: '3m', target: 1500 },
                { duration: '1m', target: 0 },
            ],
            gracefulRampDown: '0s',
        },
    },
};


export default function () {

    const url = 'wss://timer.itsblue.de/api/ws';

    const params = { tags: { my_tag: 'hello' } };

    const res = ws.connect(url, params, function (socket) {

        socket.on('open', () => {
            console.log('connected');
            socket.send(JSON.stringify({
                type: "Hello",
                data: "YIXIR",
            }));
            sleep(1)
        });

        socket.setInterval(function () {
            socket.send(JSON.stringify({
                type: "GetTime",
            }));
        }, 1000);

        socket.on('message', (data) => {
            //console.log('Message received: ', data)

            //console.log("Timestamp: ", Number(Date.now()) - Number(data.data));


        });

        socket.on('close', () => console.log('disconnected'));

    });


}