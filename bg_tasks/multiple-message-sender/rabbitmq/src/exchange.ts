import { connection } from "./services/rabbitmq/client";

const channel = await connection.createChannel();

export const exchange = "notifications";

await channel.assertExchange(exchange, "fanout", { durable: true })

export default channel;