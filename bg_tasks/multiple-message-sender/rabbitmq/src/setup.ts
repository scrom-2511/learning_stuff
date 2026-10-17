import { connection } from "./services/rabbitmq/client";

const channel = await connection.createChannel();

const queues = ["email", "discord", "telegram"];

for (const queue of queues) {
    await channel.assertQueue(queue, {
        durable: true,
        arguments: {
            "x-queue-type": "quorum"
        }
    })

    await channel.bindQueue(queue, "notifications", ""); // 3rd param is exchange routing key, empty key means fanout exchange.
}

channel.close();