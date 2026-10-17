import { deadLetterExchange } from "./exchange";
import { connection } from "./services/rabbitmq/client";

const channel = await connection.createChannel();

const queues = ["email", "discord", "telegram"];

for (const queue of queues) {
    await channel.assertQueue(queue, {
        durable: true,
        arguments: {
            "x-queue-type": "quorum",
            "x-dead-letter-exchange": deadLetterExchange,
            "x-dead-letter-routing-key": "dead"
        }
    })

    await channel.bindQueue(queue, "notifications", ""); // 3rd param is exchange routing key, empty key means fanout exchange.
}

// For dead letter queue

const deadLetterQueue = "notificationsDLQ";

await channel.assertQueue(deadLetterQueue, {
    durable: true
})

await channel.bindQueue(deadLetterQueue, deadLetterExchange, "dead");

channel.close();