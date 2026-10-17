import { connection } from "../../rabbitmq/client";

const channel = await connection.createChannel();

const queue = "telegram";

export async function consumeTelegram() {
    await channel.consume(queue, (message) => {
        console.log("recieved telegram", message?.content.toString());
    }, {
        noAck: true
    });
}

await consumeTelegram();