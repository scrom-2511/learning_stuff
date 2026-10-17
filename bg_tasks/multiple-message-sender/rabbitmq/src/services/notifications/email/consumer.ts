import { connection } from "../../rabbitmq/client";

const channel = await connection.createChannel();

const queue = "email";

export async function consumeEmail() {
    await channel.consume(queue, async (message) => {
        if (!message) return;

        console.log("Received:", message.content.toString());

        await new Promise((resolve) => setTimeout(resolve, 1000));

        channel.nack(message, true);
    }, {
        noAck: false
    });
}

await consumeEmail();