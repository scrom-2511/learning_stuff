import { connection } from "./services/rabbitmq/client";

const channel = await connection.createChannel();

export const exchange = "notifications";

await channel.assertExchange(exchange, "fanout", { durable: true })

// For Dead letter queue
export const deadLetterExchange = "notificationsDLX";

await channel.assertExchange(deadLetterExchange, "direct", { durable: true })

export default channel;