import { connection } from "../../rabbitmq/client";

const channel = await connection.createChannel();

const queue = "discord";

export async function consumeDiscord() {
    await channel.consume(queue, (message) => {
        console.log("recieved discord", message?.content.toString());
    }, {
        noAck: true
    })
}

await consumeDiscord();