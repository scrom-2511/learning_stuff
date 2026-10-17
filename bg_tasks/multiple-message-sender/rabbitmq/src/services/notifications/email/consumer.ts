import { connection } from "../../rabbitmq/client";

const channel = await connection.createChannel();

const queue = "email";

export async function consumeEmail() {
    await channel.consume(queue, (message) => {
        console.log("recieved email", message?.content.toString());
    }, {
        noAck: true
    });
}

await consumeEmail();